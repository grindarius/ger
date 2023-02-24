use actix_web::{web, HttpRequest, HttpResponse};
use tokio_postgres::types::Type;

use crate::{
    constants::{
        claims::{get_expires_timestamp, AccessTokenClaims, RefreshTokenClaims},
        requests::AuthenticationHeaders,
        responses::DefaultSuccessResponse,
        ACCESS_TOKEN_DECODING_KEY, ACCESS_TOKEN_ENCODING_KEY, ACCESS_TOKEN_HEADER_NAME,
        ACCESS_TOKEN_VALID_TIME_LENGTH, HEADER, REFRESH_TOKEN_DECODING_KEY,
        REFRESH_TOKEN_ENCODING_KEY, REFRESH_TOKEN_HEADER_NAME, REFRESH_TOKEN_VALID_TIME_LENGTH,
        VALIDATION,
    },
    database::UserSessions,
    errors::HttpError,
    shared_app_data::SharedAppData,
};

/// Checks and refresh tokens from the user when tokens are out of time.
#[utoipa::path(
    post,
    path = "/auth/refresh",
    tag = "auth",
    operation_id = "refresh",
    params(AuthenticationHeaders),
    responses(
        (
            status = 200,
            description = "refreshed successfully",
            body = DefaultSuccessResponse,
            headers(
                ("x-access-token" = String, description = "new access token"),
                ("x-refresh-token" = String, description = "new refresh token")
            ),
            example = json!(DefaultSuccessResponse::default())
        ),
        (
            status = 400,
            description = "input errors",
            body = FormattedErrorResponse,
            example = json!(HttpError::InputValidationError.get_error_struct())
        ),
        (
            status = 401,
            description = "unauthorized, any unauthorized in here requires a re-login",
            body = FormattedErrorResponse,
            example = json!(HttpError::Unauthorized.get_error_struct())
        ),
        (
            status = 500,
            description = "internal server errors",
            body = FormattedErrorResponse,
            example = json!(HttpError::InternalServerError { cause: "internal".to_string() }.get_error_struct())
        )
    )
)]
pub async fn handler(
    request: HttpRequest,
    data: web::Data<SharedAppData>,
) -> Result<HttpResponse, HttpError> {
    let client = data.pool.get().await?;

    let access_token_header = match request.headers().get(ACCESS_TOKEN_HEADER_NAME) {
        Some(t) => t,
        None => return Err(HttpError::Unauthorized),
    };
    let refresh_token_header = match request.headers().get(REFRESH_TOKEN_HEADER_NAME) {
        Some(t) => t,
        None => return Err(HttpError::Unauthorized),
    };

    if access_token_header.is_empty() || refresh_token_header.is_empty() {
        return Err(HttpError::Unauthorized);
    }

    let access_token_header = match access_token_header.to_str() {
        Ok(t) => t,
        Err(_) => return Err(HttpError::Unauthorized),
    };
    let refresh_token_header = match refresh_token_header.to_str() {
        Ok(t) => t,
        Err(_) => return Err(HttpError::Unauthorized),
    };

    let access_token = match jsonwebtoken::decode::<AccessTokenClaims>(
        access_token_header,
        &ACCESS_TOKEN_DECODING_KEY,
        &VALIDATION,
    ) {
        Ok(t) => t,
        Err(_) => return Err(HttpError::Unauthorized),
    };
    let refresh_token = match jsonwebtoken::decode::<RefreshTokenClaims>(
        refresh_token_header,
        &REFRESH_TOKEN_DECODING_KEY,
        &VALIDATION,
    ) {
        Ok(t) => t,
        Err(_) => return Err(HttpError::Unauthorized),
    };

    if access_token.claims.sid != refresh_token.claims.sid {
        client
            .execute(
                "delete from user_sessions where user_session_id in ($1, $2)",
                &[&access_token.claims.sid, &refresh_token.claims.sid],
            )
            .await?;
        return Err(HttpError::Unauthorized);
    }

    let statement = client
        .prepare_typed_cached(
            "select * from user_sessions where user_session_id = $1",
            &[Type::TEXT],
        )
        .await?;

    let session = client
        .query_one(&statement, &[&refresh_token.claims.sid])
        .await?;
    let session = UserSessions::try_from(session)?;

    let refresh_token_from_database = jsonwebtoken::decode::<RefreshTokenClaims>(
        session.user_session_refresh_token.as_str(),
        &REFRESH_TOKEN_DECODING_KEY,
        &VALIDATION,
    )?;

    if refresh_token_from_database.claims != refresh_token.claims {
        client
            .execute(
                "delete from user_sessions where user_session_refresh_token in ($1, $2)",
                &[&session.user_session_refresh_token, &refresh_token_header],
            )
            .await?;

        return Err(HttpError::Unauthorized);
    }

    let new_access_token_expires_timestamp = get_expires_timestamp(ACCESS_TOKEN_VALID_TIME_LENGTH)?;
    let new_access_token_claims = AccessTokenClaims::new(
        session.user_session_user_id.clone(),
        access_token.claims.rle,
        session.user_session_id.clone(),
        new_access_token_expires_timestamp,
    )?;

    let new_refresh_token_expires_timestamp =
        get_expires_timestamp(REFRESH_TOKEN_VALID_TIME_LENGTH)?;
    let new_refresh_token_claims = RefreshTokenClaims::new(
        session.user_session_user_id,
        session.user_session_id,
        new_refresh_token_expires_timestamp,
    )?;

    let new_access_token = jsonwebtoken::encode(
        &HEADER,
        &new_access_token_claims,
        &ACCESS_TOKEN_ENCODING_KEY,
    )?;
    let new_refresh_token = jsonwebtoken::encode(
        &HEADER,
        &new_refresh_token_claims,
        &REFRESH_TOKEN_ENCODING_KEY,
    )?;

    client
        .execute(
            r##"
            insert into user_sessions (
                user_session_id,
                user_session_user_id,
                user_session_refresh_token
            ) values (
                $1,
                $2,
                $3
            ) on conflict (user_session_id) do update set user_session_refresh_token = $3
            "##,
            &[
                &refresh_token.claims.sid,
                &refresh_token.claims.uid,
                &new_refresh_token,
            ],
        )
        .await?;

    Ok(HttpResponse::Ok()
        .insert_header((ACCESS_TOKEN_HEADER_NAME, new_access_token))
        .insert_header((REFRESH_TOKEN_HEADER_NAME, new_refresh_token))
        .json(DefaultSuccessResponse::default()))
}

#[cfg(test)]
mod tests {
    use actix_web::{http::StatusCode, test, web, App};
    use deadpool_postgres::Runtime;
    use serde_json::json;
    use tokio_postgres::NoTls;

    use crate::{
        routes::admin::signup::AdminSignupRequestBody, shared_app_data::SharedAppData,
        startup::load_postgres_config,
    };

    use super::handler;

    #[actix_web::test]
    async fn test_refresh() {
        let postgres_config = load_postgres_config();
        let pool = postgres_config
            .create_pool(Some(Runtime::Tokio1), NoTls)
            .unwrap();

        let username = "refresh_tokener";
        let email = "refresh_tokener@gmail.com";
        let password = "refresh_tokener";
        let birthdate =
            time::OffsetDateTime::now_utc() - time::Duration::new(60 * 60 * 24 * 365 * 30, 0);
        let birthdate = birthdate.date();

        let client = pool.get().await.unwrap();

        let row = client
            .query_opt(
                "select user_id from users where user_username = $1",
                &[&username],
            )
            .await
            .unwrap();

        if let Some(r) = row {
            client
                .execute("delete from users where user_username = $1", &[&username])
                .await
                .unwrap();

            client
                .execute(
                    "delete from user_sessions where user_session_user_id = $1",
                    &[&r.get::<&str, String>("user_id")],
                )
                .await
                .unwrap();
        }

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(SharedAppData::new(pool.clone())))
                .route("/refresh", web::post().to(handler))
                .route(
                    "/signup",
                    web::post().to(crate::routes::admin::signup::handler),
                )
                .route(
                    "/signin",
                    web::post().to(crate::routes::auth::signin::handler),
                ),
        )
        .await;

        let request = test::TestRequest::post()
            .uri("/signup")
            .set_json(AdminSignupRequestBody {
                username: username.to_string(),
                email: email.to_string(),
                password: password.to_string(),
                birthdate,
            })
            .to_request();

        let _response = test::call_service(&app, request).await;

        let request = test::TestRequest::post()
            .uri("/signin")
            .set_json(json!({
                "username": "refresh_tokener".to_string(),
                "password": "refresh_tokener".to_string(),
            }))
            .to_request();

        let response = test::call_service(&app, request).await;

        let access_token = response.headers().get("x-access-token").unwrap();
        let refresh_token = response.headers().get("x-refresh-token").unwrap();

        // empty access token, available refresh token
        let request = test::TestRequest::post()
            .uri("/refresh")
            .insert_header(("x-access-token", ""))
            .insert_header(("x-refresh-token", refresh_token.to_str().unwrap()))
            .to_request();
        let response = test::call_service(&app, request).await;

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

        // empty refresh token, available access token
        let request = test::TestRequest::post()
            .uri("/refresh")
            .insert_header(("x-access-token", access_token.to_str().unwrap()))
            .insert_header(("x-refresh-token", ""))
            .to_request();
        let response = test::call_service(&app, request).await;

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

        // successful call
        let request = test::TestRequest::post()
            .uri("/refresh")
            .insert_header(("x-access-token", access_token.to_str().unwrap()))
            .insert_header(("x-refresh-token", refresh_token.to_str().unwrap()))
            .to_request();
        let response = test::call_service(&app, request).await;

        assert_eq!(response.status(), StatusCode::OK);

        let row = client
            .query_opt(
                "select * from user_sessions where user_session_refresh_token = $1",
                &[&refresh_token.to_str().unwrap()],
            )
            .await
            .unwrap();

        assert!(row.is_some());
    }
}
