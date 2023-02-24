use actix_web::{web, HttpResponse};
use ger_from_row::FromRow;
use postgres_types::Type;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use utoipa::{IntoParams, ToSchema};

use crate::{
    constants::{requests::SqlRange, DEFAULT_PAGE, DEFAULT_PAGE_SIZE},
    errors::HttpError,
    shared_app_data::SharedAppData,
};

/// Which order of the replies to be applied to.
#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum GetPostRepliesRequestQueriesOrderBy {
    /// orders the replies from least recent to most recent.
    Time,
    /// orders the replies with most votes to least votes.
    Vote,
}

#[derive(Deserialize, ToSchema, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct GetPostRepliesRequestQueries {
    #[param(minimum = 1, default = json!(DEFAULT_PAGE))]
    #[serde(
        default,
        deserialize_with = "crate::constants::requests::empty_string_as_none"
    )]
    page: Option<i32>,
    #[param(minimum = 1, default = json!(DEFAULT_PAGE_SIZE))]
    #[serde(
        default,
        deserialize_with = "crate::constants::requests::empty_string_as_none"
    )]
    page_size: Option<i32>,
    /// Which kind of sorting to apply to the request
    by: Option<GetPostRepliesRequestQueriesOrderBy>,
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
    let by = query
        .by
        .as_ref()
        .unwrap_or(&GetPostRepliesRequestQueriesOrderBy::Time);
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
            {}
        limit $2
        offset $3
        "##,
        match by {
            &GetPostRepliesRequestQueriesOrderBy::Time =>
                "forum_post_replies.forum_post_reply_created_timestamp asc",
            &GetPostRepliesRequestQueriesOrderBy::Vote => "vote_count desc",
        }
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
