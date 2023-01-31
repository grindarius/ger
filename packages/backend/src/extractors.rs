use actix_web::{dev::Payload, FromRequest, HttpRequest};
use futures_util::future::{ready, Ready};

use crate::{
    constants::{
        AccessTokenClaims, RefreshTokenClaims, ACCESS_TOKEN_DECODING_KEY, ACCESS_TOKEN_HEADER_NAME,
        REFRESH_TOKEN_DECODING_KEY, REFRESH_TOKEN_HEADER_NAME, VALIDATION,
    },
    errors::HttpError,
    shared_app_data::SharedAppData,
};
use jsonwebtoken::TokenData;

pub struct AuthenticationClaims {
    pub access_token: TokenData<AccessTokenClaims>,
    pub refresh_token: TokenData<RefreshTokenClaims>,
}

impl FromRequest for AuthenticationClaims {
    type Error = HttpError;
    type Future = Ready<Result<Self, Self::Error>>;

    /// Extractor  for extracting access token and refresh token claims from the request. If these 2
    /// values are not present in any way from the request. The request is immediately responded with
    /// 401 so you need to go to refresh route.
    ///
    /// The content given from the headers have to be valid string and bytes.
    ///
    /// The function checks
    /// 1. If these 2 headers in the request exists.
    /// 2. If these 2 headers are `decode`able.
    /// 3. If both tokens have the same session id.
    /// 4. If refresh token actually exists in the database.
    ///
    /// If any of the above request fails. This function will return 401 so that user can issue new
    /// tokens.
    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let access_token_header = match req.headers().get(ACCESS_TOKEN_HEADER_NAME) {
            Some(t) => t,
            None => return ready(Err(HttpError::Unauthorized)),
        };
        let refresh_token_header = match req.headers().get(REFRESH_TOKEN_HEADER_NAME) {
            Some(t) => t,
            None => return ready(Err(HttpError::Unauthorized)),
        };

        if access_token_header.is_empty() || refresh_token_header.is_empty() {
            return ready(Err(HttpError::Unauthorized));
        }

        let access_token_header = match access_token_header.to_str() {
            Ok(t) => t,
            Err(_) => return ready(Err(HttpError::Unauthorized)),
        };

        let refresh_token_header = match refresh_token_header.to_str() {
            Ok(t) => t,
            Err(_) => return ready(Err(HttpError::Unauthorized)),
        };

        let access_token = match jsonwebtoken::decode::<AccessTokenClaims>(
            access_token_header,
            &ACCESS_TOKEN_DECODING_KEY,
            &VALIDATION,
        ) {
            Ok(t) => t,
            Err(_) => return ready(Err(HttpError::Unauthorized)),
        };

        let refresh_token = match jsonwebtoken::decode::<RefreshTokenClaims>(
            refresh_token_header,
            &REFRESH_TOKEN_DECODING_KEY,
            &VALIDATION,
        ) {
            Ok(t) => t,
            Err(_) => return ready(Err(HttpError::Unauthorized)),
        };

        let app_data = match req.app_data::<SharedAppData>() {
            Some(a) => a,
            None => return ready(Err(HttpError::Unauthorized)),
        };

        if access_token.claims.sid != refresh_token.claims.sid {}

        let current_time = time::OffsetDateTime::now_utc().unix_timestamp();
        let current_time = match current_time.try_into() {
            Ok(c) => c,
            Err(_) => return ready(Err(HttpError::Unauthorized)),
        };
        if access_token.claims.exp < current_time || refresh_token.claims.exp < current_time {
            return ready(Err(HttpError::Unauthorized));
        }

        ready(Ok(AuthenticationClaims {
            access_token,
            refresh_token,
        }))
    }
}
