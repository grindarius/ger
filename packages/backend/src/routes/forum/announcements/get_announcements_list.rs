use actix_web::HttpResponse;
use serde::Serialize;
use utoipa::ToSchema;

use crate::{
    constants::{AuthenticationHeaders, DefaultSuccessResponse},
    errors::HttpError,
    extractors::users::AuthenticatedUserClaims,
};

#[derive(Serialize, ToSchema)]
pub struct GetAnnouncementsListResponseBody {
    pub announcements: Vec<GetAnnouncementsListResponseBodyInner>,
}

#[derive(Serialize, ToSchema)]
pub struct GetAnnouncementsListResponseBodyInner {
    pub announcement_id: String,
    pub announcement_name: String,
    pub announcement_content: String,
    pub announcement_created_timestamp: time::OffsetDateTime,
}

/// Get global announcements to be shown on first page.
#[utoipa::path(
    get,
    path = "/forum/announcements",
    params(AuthenticationHeaders),
    responses(
        (
            status = 200,
            description = "successfully get list of forums",
            body = GetAnnouncementsListResponseBody,
            example = json!(DefaultSuccessResponse::default())
        ),
    )
)]
pub async fn handler(_claims: AuthenticatedUserClaims) -> Result<HttpResponse, HttpError> {
    Ok(HttpResponse::Ok().finish())
}
