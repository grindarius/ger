use std::future::{ready, Ready};

use actix_web::dev::{self, Service, ServiceRequest, ServiceResponse, Transform};
use futures_util::future::LocalBoxFuture;

use crate::constants::{
    AccessTokenClaims, RefreshTokenClaims, ACCESS_TOKEN_DECODING_KEY, ACCESS_TOKEN_HEADER_NAME,
    REFRESH_TOKEN_DECODING_KEY, REFRESH_TOKEN_HEADER_NAME, VALIDATION,
};
use crate::errors::HttpError;

pub struct CheckTokens;

impl<S, B> Transform<S, ServiceRequest> for CheckTokens
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = HttpError>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = HttpError;
    type InitError = ();
    type Transform = CheckTokensMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(CheckTokensMiddleware { service }))
    }
}

pub struct CheckTokensMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for CheckTokensMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = HttpError>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = HttpError;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future {
        let access_token_header = match request.headers().get(ACCESS_TOKEN_HEADER_NAME) {
            Some(t) => t,
            None => return Box::pin(async { Err(HttpError::InvalidAuthenticationCredentials) }),
        };
        let refresh_token_header = match request.headers().get(REFRESH_TOKEN_HEADER_NAME) {
            Some(t) => t,
            None => return Box::pin(async { Err(HttpError::InvalidAuthenticationCredentials) }),
        };

        if access_token_header.is_empty() || refresh_token_header.is_empty() {
            return Box::pin(async { Err(HttpError::InvalidAuthenticationCredentials) });
        }

        let access_token_header = match access_token_header.to_str() {
            Ok(t) => t,
            Err(_) => return Box::pin(async { Err(HttpError::InvalidAuthenticationCredentials) }),
        };
        let refresh_token_header = match refresh_token_header.to_str() {
            Ok(t) => t,
            Err(_) => return Box::pin(async { Err(HttpError::InvalidAuthenticationCredentials) }),
        };

        let access_token = match jsonwebtoken::decode::<AccessTokenClaims>(
            access_token_header,
            &ACCESS_TOKEN_DECODING_KEY,
            &VALIDATION,
        ) {
            Ok(t) => t,
            Err(_) => return Box::pin(async { Err(HttpError::InvalidAuthenticationCredentials) }),
        };
        let refresh_token = match jsonwebtoken::decode::<RefreshTokenClaims>(
            refresh_token_header,
            &REFRESH_TOKEN_DECODING_KEY,
            &VALIDATION,
        ) {
            Ok(t) => t,
            Err(_) => return Box::pin(async { Err(HttpError::InvalidAuthenticationCredentials) }),
        };

        if access_token.claims.sid != refresh_token.claims.sid {
            return Box::pin(async { Err(HttpError::Unauthorized) });
        }

        let current_time = time::OffsetDateTime::now_utc().unix_timestamp();
        let current_time: usize = match current_time.try_into() {
            Ok(c) => c,
            Err(_) => return Box::pin(async { Err(HttpError::InternalServerError) }),
        };

        if access_token.claims.exp < current_time || refresh_token.claims.exp < current_time {
            return Box::pin(async { Err(HttpError::Unauthorized) });
        }

        let fut = self.service.call(request);

        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}
