use actix_web::HttpResponse;

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
pub async fn handler() -> HttpResponse {
    HttpResponse::Ok().json(GetServerInformationResponse::default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;

    #[actix_web::test]
    async fn test_hello_ok() {
        let resp = handler().await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
