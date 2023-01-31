use actix_web::{web, HttpRequest, HttpResponse};
use tokio_postgres::types::Type;

use crate::constants::AuthenticationHeaders;
use crate::constants::{
    get_expires_timestamp, AccessTokenClaims, DefaultSuccessResponse, RefreshTokenClaims,
    ACCESS_TOKEN_DECODING_KEY, ACCESS_TOKEN_ENCODING_KEY, ACCESS_TOKEN_HEADER_NAME,
    ACCESS_TOKEN_VALID_TIME_LENGTH, HEADER, REFRESH_TOKEN_DECODING_KEY, REFRESH_TOKEN_ENCODING_KEY,
    REFRESH_TOKEN_HEADER_NAME, REFRESH_TOKEN_VALID_TIME_LENGTH, VALIDATION,
};
use crate::database::UserSessions;
use crate::errors::HttpError;
use crate::shared_app_data::SharedAppData;

// 1. check if refresh token is out of date yet. if not: refresh the token as before. if
//    yes, requires relogin.
// 2. both tokens have same session id. if not: clean the database with a given session id
//    and refresh token

#[utoipa::path(
    post,
    path = "/auth/refresh",
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
            status = 500,
            description = "internal server errors",
            body = FormattedErrorResponse,
            example = json!(HttpError::InternalServerError.get_error_struct())
        )
    )
)]
pub async fn handler(
    request: HttpRequest,
    data: web::Data<SharedAppData>,
) -> Result<HttpResponse, HttpError> {
    let client = data.pool.get().await?;

    let access_token_header = request
        .headers()
        .get(ACCESS_TOKEN_HEADER_NAME.to_string())
        .ok_or(HttpError::Unauthorized)?;
    let refresh_token_header = request
        .headers()
        .get(REFRESH_TOKEN_HEADER_NAME.to_string())
        .ok_or(HttpError::Unauthorized)?;

    if access_token_header.is_empty() || refresh_token_header.is_empty() {
        return Err(HttpError::Unauthorized);
    }

    let access_token_extracted_claims = jsonwebtoken::decode::<AccessTokenClaims>(
        access_token_header
            .to_str()
            .map_err(|_| HttpError::Unauthorized)?,
        &ACCESS_TOKEN_DECODING_KEY,
        &VALIDATION,
    )?;

    let refresh_token_extracted_claims = jsonwebtoken::decode::<RefreshTokenClaims>(
        refresh_token_header
            .to_str()
            .map_err(|_| HttpError::Unauthorized)?,
        &REFRESH_TOKEN_DECODING_KEY,
        &VALIDATION,
    )?;

    if time::OffsetDateTime::now_utc().unix_timestamp()
        > refresh_token_extracted_claims.claims.exp as i64
    {
        return Err(HttpError::Unauthorized);
    }

    let statement = client
        .prepare_typed_cached(
            "select * from user_sessions where user_session_id = $1",
            &[Type::TEXT],
        )
        .await?;

    if access_token_extracted_claims.claims.sid != refresh_token_extracted_claims.claims.sid {
        client
            .execute(
                "delete from user_sessions where user_session_id in ($1, $2)",
                &[
                    &access_token_extracted_claims.claims.sid,
                    &refresh_token_extracted_claims.claims.sid,
                ],
            )
            .await?;

        return Err(HttpError::Unauthorized);
    }

    let session = client
        .query_one(&statement, &[&refresh_token_extracted_claims.claims.sid])
        .await?;

    let session = UserSessions::try_from(session)?;

    if session.user_session_refresh_token.as_str() != refresh_token_header.to_str().unwrap() {
        client
            .execute(
                "delete from user_sessions where user_session_refresh_token in ($1, $2)",
                &[
                    &session.user_session_refresh_token,
                    &refresh_token_header.to_str().unwrap(),
                ],
            )
            .await?;

        return Err(HttpError::Unauthorized);
    }

    let new_access_token_expires_timestamp = get_expires_timestamp(ACCESS_TOKEN_VALID_TIME_LENGTH)?;
    let new_access_token_claims = AccessTokenClaims::new(
        session.user_session_user_id.clone(),
        access_token_extracted_claims.claims.rle,
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
            "update user_sessions set user_session_refresh_token = $1 where user_session_id = $2",
            &[
                &new_refresh_token,
                &refresh_token_extracted_claims.claims.sid,
            ],
        )
        .await?;

    Ok(HttpResponse::Ok()
        .insert_header((ACCESS_TOKEN_HEADER_NAME, new_access_token))
        .insert_header((REFRESH_TOKEN_HEADER_NAME, new_refresh_token))
        .json(DefaultSuccessResponse::default()))
}
