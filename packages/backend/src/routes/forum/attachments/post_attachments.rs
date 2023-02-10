use actix_web::HttpResponse;

use crate::errors::HttpError;

pub async fn handler() -> Result<HttpResponse, HttpError> {
    Ok(HttpResponse::Ok().finish())
}
