use actix_web::{HttpResponse, Responder};

use crate::constants::GetServerInformationResponse;

/// Get server information about contribution and contributors
pub async fn handler() -> impl Responder {
    HttpResponse::Ok().json(GetServerInformationResponse::default())
}
