use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::{errors::HttpError, shared_app_data::SharedAppData};

#[derive(Deserialize, ToSchema, IntoParams)]
#[into_params(parameter_in = Path)]
pub struct GetPostRepliesRequestParams {
    post_id: String,
}

#[derive(Serialize, ToSchema)]
pub struct GetPostRepliesResponseBody {
    replies: Vec<GetPostRepliesResponseBodyInner>,
}

#[derive(Serialize, ToSchema)]
pub struct GetPostRepliesResponseBodyInner {
    id: String,
    username: String,
    content: String,
    #[serde(with = "time::serde::rfc3339")]
    created_timestamp: time::OffsetDateTime,
}

#[utoipa::path(
    get,
    path = "/forum/posts/{post_id}/replies",
    tag = "forum",
    operation_id = "get_post_replies",
    params(GetPostRepliesRequestParams),
    responses(
        (
            status = 200,
            description = "successfully get list of forums",
            body = GetPostRepliesResponseBody,
            example = json!([])
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
            status = 500,
            description = "bad errors",
            body = FormattedErrorResponse,
            example = json!(HttpError::InternalServerError { cause: "internal".to_string() }.get_error_struct())
        )
    )
)]
pub async fn handler(
    params: web::Path<GetPostRepliesRequestParams>,
    data: web::Data<SharedAppData>,
) -> Result<HttpResponse, HttpError> {
    Ok(HttpResponse::Ok().finish())
}
