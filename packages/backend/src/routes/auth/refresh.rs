use actix_web::Responder;

use crate::errors::HttpError;

#[utoipa::path()]
pub async fn handler() -> Result<impl Responder, HttpError> {}
