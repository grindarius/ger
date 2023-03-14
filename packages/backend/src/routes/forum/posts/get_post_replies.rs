use actix_web::{web, HttpResponse};
use ger_from_row::FromRow;
use postgres_types::Type;
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

/// Which order of the replies to be applied to.
#[derive(Default, Serialize, Deserialize, ToSchema, TS, Clone, Copy)]
#[serde(rename_all = "lowercase")]
#[ts(export)]
pub enum GetPostRepliesRequestQueriesOrderBy {
    /// orders the replies from least recent to most recent.
    #[default]
    Time,
    /// orders the replies with most votes to least votes.
    Vote,
}

pub struct GetPostRepliesRequestQueriesOrderByModifier;

impl Modify for GetPostRepliesRequestQueriesOrderByModifier {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        openapi.components.as_mut().map(|v| {
            v.schemas
                .get_mut("GetPostRepliesRequestQueriesOrderBy")
                .map(|z| {
                    if let RefOr::T(schema) = z {
                        if let Schema::Object(obj) = schema {
                            obj.default = Some(json!(to_variant_name(
                                &GetPostRepliesRequestQueriesOrderBy::default()
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
pub struct GetPostRepliesRequestQueries {
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
    /// Which kind of sorting to apply to the response.
    #[param(default = json!(GetPostRepliesRequestQueriesOrderBy::default()))]
    #[ts(optional)]
    by: Option<GetPostRepliesRequestQueriesOrderBy>,
    /// Specify the order of the data that you wanted to query.
    #[param(default = json!(Order::default()))]
    #[ts(optional)]
    order: Option<Order>,
}

#[derive(Deserialize, ToSchema, IntoParams)]
#[into_params(parameter_in = Path)]
pub struct GetPostRepliesRequestParams {
    post_id: String,
}

#[derive(Serialize, ToSchema, TS)]
#[ts(export)]
pub struct GetPostRepliesResponseBody {
    replies: Vec<GetPostRepliesResponseBodyInner>,
}

#[derive(FromRow, Serialize, ToSchema, TS)]
#[ts(export)]
pub struct GetPostRepliesResponseBodyInner {
    id: String,
    user_id: String,
    username: String,
    /// Parsed `markdown` content
    content: String,
    #[serde(with = "time::serde::rfc3339")]
    #[ts(type = "string")]
    created_timestamp: time::OffsetDateTime,
    #[schema(value_type = String)]
    #[serde(serialize_with = "crate::constants::responses::serialize_bigint_to_string")]
    #[ts(type = "string")]
    vote_count: i64,
}

/// Get replies list of a given `post_id`
#[utoipa::path(
    get,
    path = "/forum/posts/{post_id}/replies",
    tag = "forum",
    operation_id = "get_post_replies",
    params(GetPostRepliesRequestParams, GetPostRepliesRequestQueries),
    responses(
        (
            status = 200,
            description = "successfully get list of forum replies",
            body = GetPostRepliesResponseBody,
            example = json!({ "replies": [] })
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
    params: web::Path<GetPostRepliesRequestParams>,
    query: web::Query<GetPostRepliesRequestQueries>,
    data: web::Data<SharedAppData>,
) -> Result<HttpResponse, HttpError> {
    let page = query.page.unwrap_or(DEFAULT_PAGE);
    let page_size = query.page_size.unwrap_or(DEFAULT_PAGE_SIZE);
    let by = query.by.unwrap_or_default();
    let order = query.order.unwrap_or_default();
    let SqlRange { limit, offset } = SqlRange::from_page(page, page_size)?;

    let client = data.pool.get().await?;
    let query_string = format!(
        r##"
        select
            forum_post_replies.forum_post_reply_id as id,
            forum_post_replies.user_id as user_id,
            users.user_username as username,
            forum_post_replies.forum_post_reply_content as content,
            forum_post_replies.forum_post_reply_created_timestamp as created_timestamp,
            sum(forum_post_reply_votes.forum_post_reply_vote_increment) as vote_count
        from forum_post_replies
        inner join users on users.user_id = forum_post_replies.user_id
        inner join forum_post_reply_votes on forum_post_replies.forum_post_reply_id = forum_post_reply_votes.forum_post_reply_id
        where
            forum_post_replies.forum_post_id = $1
        group by
            forum_post_replies.forum_post_reply_id,
            users.user_username
        order by
            {} {}
        limit $2
        offset $3
        "##,
        match by {
            GetPostRepliesRequestQueriesOrderBy::Time =>
                "forum_post_replies.forum_post_reply_created_timestamp",
            GetPostRepliesRequestQueriesOrderBy::Vote => "vote_count",
        },
        to_variant_name(&order).unwrap()
    );

    let statement = client
        .prepare_typed_cached(query_string.as_str(), &[Type::TEXT, Type::INT4, Type::INT4])
        .await?;

    let replies = client
        .query(&statement, &[&params.post_id, &limit, &offset])
        .await?;

    let replies = replies
        .iter()
        .map(|r| GetPostRepliesResponseBodyInner::try_from(r))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(HttpResponse::Ok().json(GetPostRepliesResponseBody { replies }))
}
