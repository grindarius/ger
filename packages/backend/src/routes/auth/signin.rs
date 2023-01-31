use actix_web::{web, HttpResponse};
use argon2::{PasswordHash, PasswordVerifier};
use serde::Deserialize;
use tokio_postgres::types::Type;
use utoipa::ToSchema;

use crate::{
    constants::{
        create_argon2_context, get_expires_timestamp, AccessTokenClaims, DefaultSuccessResponse,
        RefreshTokenClaims, Role, ACCESS_TOKEN_ENCODING_KEY, ACCESS_TOKEN_HEADER_NAME,
        ACCESS_TOKEN_VALID_TIME_LENGTH, HEADER, ID_LENGTH, REFRESH_TOKEN_ENCODING_KEY,
        REFRESH_TOKEN_HEADER_NAME, REFRESH_TOKEN_VALID_TIME_LENGTH,
    },
    errors::HttpError,
    shared_app_data::SharedAppData,
};

#[derive(Deserialize, ToSchema)]
pub struct SigninBody {
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
    request_body = SigninBody,
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
    body: web::Json<SigninBody>,
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
