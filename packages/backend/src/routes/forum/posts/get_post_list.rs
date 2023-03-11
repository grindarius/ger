use actix_web::{web, HttpResponse};
use ger_from_row::FromRow;
use postgres_types::{ToSql, Type};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_variant::to_variant_name;
use ts_rs::TS;
use utoipa::{
    openapi::{RefOr, Schema},
    IntoParams, Modify, ToSchema,
};

use crate::{
    constants::{
        requests::{Order, SqlRange},
        DEFAULT_PAGE, DEFAULT_PAGE_SIZE,
    },
    errors::HttpError,
    shared_app_data::SharedAppData,
};

/// Specify either how to sort the api by each order
#[derive(Default, Deserialize, Serialize, ToSchema, TS, Clone, Copy)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum GetPostListRequestQueriesOrderBy {
    /// Sort with latest activity
    #[default]
    LatestActivity,
    /// Sort with amount of votes
    Vote,
    /// Sort with created time
    Time,
    /// Sort with amount of views
    View,
}

pub struct GetPostListRequestQueriesOrderByModifier;

impl Modify for GetPostListRequestQueriesOrderByModifier {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        openapi.components.as_mut().map(|v| {
            v.schemas
                .get_mut("GetPostListRequestQueriesOrderBy")
                .map(|z| {
                    if let RefOr::T(schema) = z {
                        if let Schema::Object(obj) = schema {
                            obj.default = Some(json!(to_variant_name(
                                &GetPostListRequestQueriesOrderBy::default()
                            )
                            .unwrap()))
                        }
                    }
                })
        });
    }
}

#[derive(Deserialize, ToSchema, IntoParams, TS)]
#[into_params(parameter_in = Query)]
#[ts(export)]
pub struct GetPostListRequestQueries {
    /// get list of global announcements. If this value is `true`, this will ignore any other
    /// fields.
    #[param(default = json!(false))]
    #[ts(optional)]
    pub announcement: Option<bool>,
    /// get list of category based announcements
    #[param(default = json!(false))]
    #[ts(optional)]
    pub category_based_announcement: Option<bool>,
    /// specify how to sort the response
    #[param(default = json!(GetPostListRequestQueriesOrderBy::default()))]
    #[ts(optional)]
    pub by: Option<GetPostListRequestQueriesOrderBy>,
    /// specify how to order the response
    #[param(default = json!(Order::default()))]
    #[ts(optional)]
    pub order: Option<Order>,
    /// specify the `category_representative_id` of the post that you would like to query.
    #[param(example = json!("uncategorized"), default = json!(""))]
    #[serde(
        default,
        deserialize_with = "crate::constants::requests::empty_string_as_none"
    )]
    #[ts(optional)]
    pub category_representative_id: Option<String>,
    /// specify only active or inactive posts. If not specified, will take both active and inactive
    /// posts.
    #[param(default = json!(true))]
    #[ts(optional)]
    pub active: Option<bool>,
    /// page of the queried data. If less than `1`, will default to `1`
    #[param(minimum = 1, default = json!(DEFAULT_PAGE))]
    #[serde(deserialize_with = "crate::constants::requests::deserialize_page")]
    #[ts(optional)]
    pub page: Option<i32>,
    /// size of page for each query. will use default page size if it is out of bounds.
    #[param(minimum = 1, maximum = 100, default = json!(DEFAULT_PAGE_SIZE))]
    #[serde(deserialize_with = "crate::constants::requests::deserialize_page_size")]
    #[ts(optional)]
    pub page_size: Option<i32>,
}

#[derive(Serialize, ToSchema, TS)]
#[ts(export)]
pub struct GetPostListResponseBody {
    posts: Vec<GetPostListResponseBodyInner>,
}

#[derive(FromRow, Serialize, ToSchema, TS)]
#[ts(export)]
pub struct GetPostListResponseBodyInner {
    id: String,
    user_id: String,
    username: String,
    name: String,
    #[serde(with = "time::serde::rfc3339")]
    #[ts(type = "string")]
    created_timestamp: time::OffsetDateTime,
    category_id: String,
    category_representative_id: String,
    #[ts(type = "number")]
    view_count: i64,
    #[ts(type = "number")]
    vote_count: i64,
    #[ts(type = "number")]
    reply_count: i64,
    is_active: bool,
    #[serde(with = "time::serde::rfc3339::option")]
    #[ts(type = "string")]
    deactivated_timestamp: Option<time::OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339")]
    #[ts(type = "string")]
    last_active_timestamp: time::OffsetDateTime,
}

/// Get a list of posts to be displayed in many forum pages.
#[utoipa::path(
    get,
    path = "/forum/posts",
    tag = "forum",
    operation_id = "get_post_list",
    params(GetPostListRequestQueries),
    responses(
        (
            status = 200,
            description = "successfully get list of posts",
            body = GetPostListResponseBody,
            example = json!({ "posts": [] })
        ),
        (
            status = 400,
            description = "input errors",
            body = FormattedErrorResponse,
            example = json!(HttpError::InputValidationError.get_error_struct())
        ),
        (
            status = 401,
            description = "unauthorized",
            body = FormattedErrorResponse,
            example = json!(HttpError::Unauthorized.get_error_struct())
        ),
        (
            status = 500,
            description = "bad errors",
            body = FormattedErrorResponse,
            example = json!(HttpError::InternalServerError { cause: "internal".to_string() }.get_error_struct())
        )
    )
)]
pub async fn handler(
    query: web::Query<GetPostListRequestQueries>,
    data: web::Data<SharedAppData>,
) -> Result<HttpResponse, HttpError> {
    let category = query.category_representative_id.clone();
    let announcement = query.announcement.unwrap_or(false);
    let category_based_announcement = query.category_based_announcement.unwrap_or(false);
    let by = query.by.unwrap_or_default();
    let active = query.active;
    let order = query.order.unwrap_or_default();

    // Safe unwrap for both of the query params because the deserializer does not emit `None`
    let page = query.page.unwrap();
    let page_size = query.page_size.unwrap();

    let SqlRange { limit, offset } = SqlRange::from_page(page, page_size)?;

    let client = data.pool.get().await?;

    let statement_query_string = format!(
        r##"
        select
            forum_posts.forum_post_id as id,
            forum_posts.user_id as user_id,
            users.user_username as username,
            forum_posts.forum_post_name as name,
            forum_posts.forum_category_id as category_id,
            forum_categories.forum_category_representative_id as category_representative_id,
            count(distinct forum_post_views.user_id) as view_count,
            sum(forum_post_votes.forum_post_vote_increment) as vote_count,
            forum_posts.forum_post_created_timestamp as created_timestamp,
            forum_posts.forum_post_last_active_timestamp as last_active_timestamp,
            count(distinct forum_post_replies.forum_post_reply_id) as reply_count,
            forum_posts.forum_post_is_active as is_active,
            forum_posts.forum_post_deactivated_timestamp as deactivated_timestamp
        from forum_posts
        inner join users on forum_posts.user_id = users.user_id
        inner join forum_post_views on forum_posts.forum_post_id = forum_post_views.forum_post_id
        inner join forum_post_votes on forum_posts.forum_post_id = forum_post_votes.forum_post_id
        inner join forum_categories on forum_posts.forum_category_id = forum_categories.forum_category_id
        inner join forum_post_replies on forum_posts.forum_post_id = forum_post_replies.forum_post_id
        where
            forum_posts.forum_post_is_global_announcement = $1 and
            forum_posts.forum_post_is_category_based_announcement = $2
            {}
            {}
        group by
            forum_posts.forum_post_id,
            users.user_username,
            forum_categories.forum_category_representative_id
        order by
            {} {}
        limit $3
        offset $4
        "##,
        match category {
            Some(_) => "and forum_categories.forum_category_representative_id = $5",
            None => "",
        },
        match active {
            Some(_) => "and forum_posts.forum_post_is_active = $6",
            None => "",
        },
        match by {
            GetPostListRequestQueriesOrderBy::Time => "forum_posts.forum_post_created_timestamp",
            GetPostListRequestQueriesOrderBy::LatestActivity =>
                "forum_posts.forum_post_last_active_timestamp",
            GetPostListRequestQueriesOrderBy::Vote => "vote_count",
            GetPostListRequestQueriesOrderBy::View => "view_count",
        },
        to_variant_name(&order)?
    );

    let statement = client
        .prepare_typed_cached(
            &statement_query_string,
            &[
                Type::BOOL,
                Type::BOOL,
                Type::INT4,
                Type::INT4,
                Type::TEXT,
                Type::BOOL,
            ],
        )
        .await?;

    let query_params: [&(dyn ToSql + Sync); 6] = [
        &announcement,
        &category_based_announcement,
        &limit,
        &offset,
        &category,
        &active,
    ];

    let posts = client.query(&statement, &query_params).await?;

    let posts = posts
        .iter()
        .map(|p| GetPostListResponseBodyInner::try_from(p))
        .collect::<Result<Vec<_>, _>>()?;

    return Ok(HttpResponse::Ok().json(GetPostListResponseBody { posts }));
}
