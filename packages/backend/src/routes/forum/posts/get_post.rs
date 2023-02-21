use actix_web::{web, HttpResponse};
use comrak::markdown_to_html;
use ger_from_row::FromRow;
use postgres_types::Type;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use utoipa::{IntoParams, ToSchema};

use crate::{constants::COMRAK_OPTIONS, errors::HttpError, shared_app_data::SharedAppData};

#[derive(Deserialize, ToSchema, IntoParams)]
#[into_params(parameter_in = Path)]
pub struct GetPostRequestParams {
    pub post_id: String,
}

#[derive(Serialize, ToSchema, FromRow, TS)]
#[ts(export)]
pub struct GetPostResponseBody {
    id: String,
    user_id: String,
    username: String,
    name: String,
    /// HTML parsed content using `comrak`.
    content: String,
    #[serde(with = "time::serde::rfc3339")]
    #[ts(type = "string")]
    created_timestamp: time::OffsetDateTime,
    category_id: String,
    category_representative_id: String,
    view_count: i64,
    vote_count: i64,
}

impl Default for GetPostResponseBody {
    fn default() -> Self {
        Self {
            id: "xlKFXqgUNkeGUnVNZvqrrFD0TtVm3-EU".to_string(),
            user_id: "4QMPeOBJnjdcl6tMPllRtWHZoisfj9nU".to_string(),
            username: "grindarius".to_string(),
            name: "How to surf the web".to_string(),
            content: "<p>go on gooogle.com</p>".to_string(),
            created_timestamp: time::OffsetDateTime::from_unix_timestamp(1_546_600_000).unwrap(),
            category_id: "EmYYiZHI_P2ZdyH34p_S3t5Lwq8eENJX".to_string(),
            category_representative_id: "homework".to_string(),
            view_count: 15,
            vote_count: 30,
        }
    }
}

#[utoipa::path(
    get,
    path = "/forum/posts/{post_id}",
    tag = "forum",
    operation_id = "get_post",
    params(GetPostRequestParams),
    responses(
        (
            status = 200,
            description = "successfully get list of forums",
            body = GetPostResponseBody,
            example = json!(GetPostResponseBody::default())
        ),
        (
            status = 400,
            description = "input erorrs",
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
            status = 404,
            description = "post not found",
            body = FormattedErrorResponse,
            example = json!(HttpError::PostNotFound.get_error_struct())
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
    params: web::Path<GetPostRequestParams>,
    data: web::Data<SharedAppData>,
) -> Result<HttpResponse, HttpError> {
    let client = data.pool.get().await?;

    let statement = client
        .prepare_typed_cached(
            r##"
            select
                forum_posts.forum_post_id as id,
                forum_posts.user_id as user_id,
                users.user_username as username,
                forum_posts.forum_post_name as name,
                forum_posts.forum_post_content as content,
                forum_posts.forum_post_created_timestamp as created_timestamp,
                forum_posts.forum_category_id as category_id,
                forum_categories.forum_category_representative_id as category_representative_id,
                count(distinct forum_post_views.user_id) as view_count,
                sum(forum_post_votes.forum_post_vote_increment) as vote_count
            from forum_posts
            inner join users on forum_posts.user_id = users.user_id
            inner join forum_post_views on forum_posts.forum_post_id = forum_post_views.forum_post_id
            inner join forum_post_votes on forum_posts.forum_post_id = forum_post_votes.forum_post_id
            inner join forum_categories on forum_posts.forum_category_id = forum_categories.forum_category_id
            where
                forum_posts.forum_post_id = $1
            group by
                forum_posts.forum_post_id,
                users.user_username,
                forum_categories.forum_category_representative_id
            "##,
            &[Type::TEXT],
        )
        .await?;

    let row = client.query_opt(&statement, &[&params.post_id]).await?;

    if let Some(r) = row {
        let raw_content = r.try_get::<&str, String>("content")?;
        let parsed_content = markdown_to_html(&raw_content.as_str(), &COMRAK_OPTIONS);

        let post = GetPostResponseBody {
            id: r.try_get::<&str, String>("id")?,
            user_id: r.try_get::<&str, String>("user_id")?,
            username: r.try_get::<&str, String>("username")?,
            name: r.try_get::<&str, String>("name")?,
            category_id: r.try_get::<&str, String>("category_id")?,
            category_representative_id: r.try_get::<&str, String>("category_representative_id")?,
            content: parsed_content,
            created_timestamp: r.try_get::<&str, time::OffsetDateTime>("created_timestamp")?,
            view_count: r.try_get::<&str, i64>("view_count")?,
            vote_count: r.try_get::<&str, i64>("vote_count")?,
        };

        return Ok(HttpResponse::Ok().json(post));
    }

    Err(HttpError::PostNotFound)
}
