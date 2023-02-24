use actix_web::{web, HttpResponse};
use ger_from_row::FromRow;
use postgres_types::Type;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use utoipa::{IntoParams, ToSchema};

use crate::{
    constants::{
        requests::{Order, SqlRange},
        DEFAULT_PAGE, DEFAULT_PAGE_SIZE,
    },
    errors::HttpError,
    shared_app_data::SharedAppData,
};

/// Specify either how to sort the api by each order
#[derive(Deserialize, Serialize, ToSchema, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum GetPostListRequestQueriesOrderBy {
    /// Sort with latest activity
    LatestActivity,
    /// Sort with amount of votes
    Vote,
    /// Sort with created time
    Time,
    /// Sort with amount of views
    View,
}

#[derive(Deserialize, ToSchema, IntoParams, TS)]
#[into_params(parameter_in = Query)]
#[ts(export)]
pub struct GetPostListRequestQueries {
    /// get list of global announcements
    #[param(default = json!(false))]
    #[ts(optional)]
    pub announcement: Option<bool>,
    /// get list of category based announcements
    #[param(default = json!(false))]
    #[ts(optional)]
    pub category_based_announcement: Option<bool>,
    /// specify how to sort the response
    #[param(default = json!(GetPostListRequestQueriesOrderBy::LatestActivity))]
    #[ts(optional)]
    pub by: Option<GetPostListRequestQueriesOrderBy>,
    /// specify how to order the response
    #[param(default = json!(Order::Asc))]
    #[ts(optional)]
    pub order: Option<Order>,
    /// page of the queried data
    #[param(minimum = 1, default = json!(DEFAULT_PAGE))]
    #[serde(
        default,
        deserialize_with = "crate::constants::requests::empty_string_as_none"
    )]
    #[ts(optional)]
    pub page: Option<i32>,
    /// size of page for each query
    #[param(minimum = 1, default = json!(DEFAULT_PAGE_SIZE))]
    #[serde(
        default,
        deserialize_with = "crate::constants::requests::empty_string_as_none"
    )]
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
    #[serde(with = "time::serde::rfc3339")]
    #[ts(type = "string")]
    last_active_timestamp: time::OffsetDateTime,
}

/// Get a list of posts to be displayed in the forum's main page.
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
    let announcement = query.announcement.unwrap_or(false);
    let category_based_announcement = query.category_based_announcement.unwrap_or(false);
    let by = query
        .by
        .as_ref()
        .unwrap_or(&GetPostListRequestQueriesOrderBy::LatestActivity);
    let order = query.order.as_ref().unwrap_or(&Order::Asc);
    let page = query.page.unwrap_or(DEFAULT_PAGE);
    let page_size = query.page_size.unwrap_or(DEFAULT_PAGE_SIZE);

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
            count(distinct forum_post_replies.forum_post_reply_id) as reply_count
        from forum_posts
        inner join users on forum_posts.user_id = users.user_id
        inner join forum_post_views on forum_posts.forum_post_id = forum_post_views.forum_post_id
        inner join forum_post_votes on forum_posts.forum_post_id = forum_post_votes.forum_post_id
        inner join forum_categories on forum_posts.forum_category_id = forum_categories.forum_category_id
        inner join forum_post_replies on forum_posts.forum_post_id = forum_post_replies.forum_post_id
        where
            forum_posts.forum_post_is_global_announcement = $1 and
            forum_posts.forum_post_is_category_based_announcement = $2
        group by
            forum_posts.forum_post_id,
            users.user_username,
            forum_categories.forum_category_representative_id
        order by {} {}
        limit $3
        offset $4
        "##,
        match by {
            GetPostListRequestQueriesOrderBy::Time => "forum_posts.forum_post_created_timestamp",
            GetPostListRequestQueriesOrderBy::LatestActivity =>
                "forum_posts.forum_post_last_active_timestamp",
            GetPostListRequestQueriesOrderBy::Vote => "vote_count",
            GetPostListRequestQueriesOrderBy::View => "view_count",
        },
        order.to_string(),
    );

    let statement = client
        .prepare_typed_cached(
            &statement_query_string,
            &[Type::BOOL, Type::BOOL, Type::INT4, Type::INT4],
        )
        .await?;

    let posts = client
        .query(
            &statement,
            &[&announcement, &category_based_announcement, &limit, &offset],
        )
        .await?;

    let posts = posts
        .iter()
        .map(|p| GetPostListResponseBodyInner::try_from(p))
        .collect::<Result<Vec<_>, _>>()?;

    return Ok(HttpResponse::Ok().json(GetPostListResponseBody { posts }));
}
