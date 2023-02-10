use actix_web::{web, HttpResponse};
use argon2::{PasswordHash, PasswordVerifier};
use serde::{Deserialize, Serialize};
use tokio_postgres::types::Type;
use utoipa::ToSchema;

use crate::{
    constants::{
        claims::{AccessTokenClaims, RefreshTokenClaims},
        create_argon2_context, get_expires_timestamp,
        responses::DefaultSuccessResponse,
        ACCESS_TOKEN_ENCODING_KEY, ACCESS_TOKEN_HEADER_NAME, ACCESS_TOKEN_VALID_TIME_LENGTH,
        HEADER, ID_LENGTH, REFRESH_TOKEN_ENCODING_KEY, REFRESH_TOKEN_HEADER_NAME,
        REFRESH_TOKEN_VALID_TIME_LENGTH,
    },
    database::Role,
    errors::HttpError,
    shared_app_data::SharedAppData,
};

#[derive(Serialize, Deserialize, ToSchema)]
pub struct SigninRequestBody {
    pub username: String,
    pub password: String,
}

#[derive(ger_from_row::FromRow)]
pub struct UserQuery {
    user_id: String,
    user_password: String,
    #[fromrow(num)]
    user_role: Role,
}

/// Signs user into the website, returns access token and refresh token for user to login further.
#[utoipa::path(
    post,
    path = "/auth/signin",
    tag = "auth",
    operation_id = "signin",
    request_body = SigninRequestBody,
    responses(
        (
            status = 200,
            description = "Success call",
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
            status = 500,
            description = "internal server errors",
            body = FormattedErrorResponse,
            example = json!(HttpError::InternalServerError.get_error_struct())
        )
    )
)]
pub async fn handler(
    body: web::Json<SigninRequestBody>,
    data: web::Data<SharedAppData>,
) -> Result<HttpResponse, HttpError> {
    if body.username.is_empty() {
        return Err(HttpError::InputValidationError);
    }

    if body.password.is_empty() {
        return Err(HttpError::InputValidationError);
    }

    let client = data.pool.get().await?;

    let statement = client
        .prepare_typed_cached(
            r##"
            select
                users.user_id,
                users.user_password,
                users.user_role
            from users
            where users.user_username = $1"##,
            &[Type::TEXT],
        )
        .await?;

    let user = client.query_one(&statement, &[&body.username]).await?;
    let user = UserQuery::try_from(&user)?;

    let parsed_password = PasswordHash::new(user.user_password.as_str())?;
    let password_result = create_argon2_context()?
        .verify_password(body.password.as_bytes(), &parsed_password)
        .is_ok();

    if !password_result {
        return Err(HttpError::IncorrectPassword);
    }

    let new_session_id = nanoid::nanoid!(ID_LENGTH);

    let access_token_expires_timestamp = get_expires_timestamp(ACCESS_TOKEN_VALID_TIME_LENGTH)?;
    let access_token_claims = AccessTokenClaims::new(
        Clone::clone(&user.user_id),
        user.user_role,
        Clone::clone(&new_session_id),
        access_token_expires_timestamp,
    )?;

    let refresh_token_expires_timestamp = get_expires_timestamp(REFRESH_TOKEN_VALID_TIME_LENGTH)?;
    let refresh_token_claims = RefreshTokenClaims::new(
        Clone::clone(&user.user_id),
        Clone::clone(&new_session_id),
        refresh_token_expires_timestamp,
    )?;

    let access_token =
        jsonwebtoken::encode(&HEADER, &access_token_claims, &ACCESS_TOKEN_ENCODING_KEY)?;
    let refresh_token =
        jsonwebtoken::encode(&HEADER, &refresh_token_claims, &REFRESH_TOKEN_ENCODING_KEY)?;

    client
        .execute(
            "insert into user_sessions (user_session_id, user_session_user_id, user_session_refresh_token) values ($1, $2, $3)",
            &[&new_session_id, &user.user_id, &refresh_token])
        .await?;

    Ok(HttpResponse::Ok()
        .insert_header((ACCESS_TOKEN_HEADER_NAME, access_token))
        .insert_header((REFRESH_TOKEN_HEADER_NAME, refresh_token))
        .json(DefaultSuccessResponse::default()))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::load_postgres_config;

    use actix_web::{http::StatusCode, test, App};
    use argon2::{password_hash::SaltString, PasswordHasher};
    use deadpool_postgres::Runtime;
    use rand_core::OsRng;
    use tokio_postgres::NoTls;

    #[actix_web::test]
    async fn signin() {
        let pool = load_postgres_config()
            .create_pool(Some(Runtime::Tokio1), NoTls)
            .expect("cannot create testing pool");

        let client = pool.get().await.unwrap();

        // 30 years old
        let birthdate =
            time::OffsetDateTime::now_utc() - time::Duration::new(30 * 365 * 24 * 60 * 60, 0);

        let username = "simple_user_signin".to_string();
        let email = "simple_user_signin@gmail.com".to_string();

        let salt = SaltString::generate(&mut OsRng);
        let argon2_context = create_argon2_context().unwrap();
        let password = argon2_context.hash_password(b"aryastark", &salt).unwrap();

        client
            .execute(
                r##"
                insert into users (
                    user_id,
                    user_username,
                    user_email,
                    user_password,
                    user_role,
                    user_birthdate
                ) values (
                    $1,
                    $2,
                    $3,
                    $4,
                    $5,
                    $6
                )
                "##,
                &[
                    &nanoid::nanoid!(ID_LENGTH),
                    &username,
                    &email,
                    &password.to_string(),
                    &Role::Admin,
                    &birthdate.date(),
                ],
            )
            .await
            .unwrap();

        let app = test::init_service(
            App::new()
                .app_data(SharedAppData::new(pool.clone()))
                .route("/", web::post().to(handler)),
        )
        .await;

        // empty username
        let request = test::TestRequest::post()
            .uri("/")
            .set_json(SigninRequestBody {
                username: "".to_string(),
                password: password.clone().to_string(),
            })
            .to_request();
        let response = test::call_service(&app, request).await;

        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        // empty password
        let request = test::TestRequest::post()
            .uri("/")
            .set_json(SigninRequestBody {
                username: username.clone(),
                password: "".to_string(),
            })
            .to_request();
        let response = test::call_service(&app, request).await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        // user not found
        let request = test::TestRequest::post()
            .uri("/")
            .set_json(SigninRequestBody {
                username: "who_is_this_eh".to_string(),
                password: "who_is_this_eh".to_string(),
            })
            .to_request();
        let response = test::call_service(&app, request).await;

        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        // wrong password
        let request = test::TestRequest::post()
            .uri("/")
            .set_json(SigninRequestBody {
                username: username.clone().to_string(),
                password: password.clone().to_string(),
            })
            .to_request();
        let response = test::call_service(&app, request).await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        // successful
        let request = test::TestRequest::post()
            .uri("/")
            .set_json(SigninRequestBody {
                username: username.clone().to_string(),
                password: password.clone().to_string(),
            })
            .to_request();
        let response = test::call_service(&app, request).await;

        assert_eq!(response.status(), StatusCode::OK);
        assert_ne!(
            response.headers().get("x-access-token"),
            None,
            "checking that the headers gets created"
        );
        assert_ne!(
            response.headers().get("x-refresh-token"),
            None,
            "checking that the headers gets created"
        );
    }
}
