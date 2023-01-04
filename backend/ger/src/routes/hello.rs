use actix_web::{HttpResponse, Responder};

use crate::constants::GetServerInformationResponse;

/// Get server information about contribution and contributors
#[utoipa::path(
    get,
    path = "/",
    responses(
        (
            status = 200,
            body = GetServerInformationResponse,
            example = json!(GetServerInformationResponse::default())
        )
    )
)]
pub async fn handler() -> impl Responder {
    HttpResponse::Ok().json(GetServerInformationResponse::default())
}
