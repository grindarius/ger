use actix_web::HttpRequest;
use jsonwebtoken::TokenData;

use crate::{
    constants::{
        claims::{AccessTokenClaims, RefreshTokenClaims},
        ACCESS_TOKEN_DECODING_KEY, ACCESS_TOKEN_HEADER_NAME, REFRESH_TOKEN_DECODING_KEY,
        REFRESH_TOKEN_HEADER_NAME, VALIDATION,
    },
    errors::HttpError,
};

pub struct AuthenticatedClaims {
    pub access_token: TokenData<AccessTokenClaims>,
    pub refresh_token: TokenData<RefreshTokenClaims>,
}

pub mod admins;
pub mod users;

/// A function to check whether a token is valid inside of the headers.
///
/// This function checks whether
/// 1. both tokens exist in the header.
/// 2. both tokens are valid.
/// 3. both tokens are able to produce claims.
/// 4. both tokens are not yet expired.
/// 5. both tokens have the same session id.
///
/// If an error were to happen at number 1, 2, and 3. The extractor will produce [400
/// InvalidAuthenticationCredentials](crate::errors::HttpError) so that user can go to re login again.
///
/// If an error were to happen at number 4 and 5 instead. The extractor will produce [401
/// Unauthorized](crate::errors::HttpError) so that user can be redirected to refresh the tokens.
pub fn validate_tokens_in_header(request: &HttpRequest) -> Result<AuthenticatedClaims, HttpError> {
    let access_token_header = match request.headers().get(ACCESS_TOKEN_HEADER_NAME) {
        Some(t) => t,
        None => return Err(HttpError::InvalidAuthenticationCredentials),
    };
    let refresh_token_header = match request.headers().get(REFRESH_TOKEN_HEADER_NAME) {
        Some(t) => t,
        None => return Err(HttpError::InvalidAuthenticationCredentials),
    };

    if access_token_header.is_empty() || refresh_token_header.is_empty() {
        return Err(HttpError::InvalidAuthenticationCredentials);
    }

    let access_token_header = match access_token_header.to_str() {
        Ok(t) => t,
        Err(_) => return Err(HttpError::InvalidAuthenticationCredentials),
    };
    let refresh_token_header = match refresh_token_header.to_str() {
        Ok(t) => t,
        Err(_) => return Err(HttpError::InvalidAuthenticationCredentials),
    };

    let access_token = match jsonwebtoken::decode::<AccessTokenClaims>(
        access_token_header,
        &ACCESS_TOKEN_DECODING_KEY,
        &VALIDATION,
    ) {
        Ok(t) => t,
        Err(_) => return Err(HttpError::InvalidAuthenticationCredentials),
    };
    let refresh_token = match jsonwebtoken::decode::<RefreshTokenClaims>(
        refresh_token_header,
        &REFRESH_TOKEN_DECODING_KEY,
        &VALIDATION,
    ) {
        Ok(t) => t,
        Err(_) => return Err(HttpError::InvalidAuthenticationCredentials),
    };

    if access_token.claims.sid != refresh_token.claims.sid {
        return Err(HttpError::Unauthorized);
    }

    let current_time = time::OffsetDateTime::now_utc().unix_timestamp();
    let current_time: usize = match current_time.try_into() {
        Ok(c) => c,
        Err(_) => return Err(HttpError::InternalServerError),
    };

    if access_token.claims.exp < current_time || refresh_token.claims.exp < current_time {
        return Err(HttpError::Unauthorized);
    }

    Ok(AuthenticatedClaims {
        access_token,
        refresh_token,
    })
}
