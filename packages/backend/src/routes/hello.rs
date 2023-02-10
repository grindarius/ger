use actix_web::HttpResponse;

use crate::constants::responses::GetServerInformationResponse;

/// Get server information about contribution and contributors
#[utoipa::path(
    get,
    path = "/",
    tag = "home",
    operation_id = "get_server_information",
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
    use actix_web::{http::StatusCode, test, web, App};

    #[actix_web::test]
    async fn test_hello_ok() {
        let response = handler().await;
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_hello_404() {
        let app = test::init_service(App::new().route("/", web::get().to(handler))).await;
        let request = test::TestRequest::post().uri("/").to_request();
        let response = test::call_service(&app, request).await;

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
