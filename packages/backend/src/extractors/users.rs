use actix_web::{dev::Payload, FromRequest, HttpRequest};
use futures_util::future::{ready, Ready};

use crate::errors::HttpError;

use super::{validate_tokens_in_header, AuthenticatedClaims};

pub struct AuthenticatedUserClaims(pub AuthenticatedClaims);

impl FromRequest for AuthenticatedUserClaims {
    type Error = HttpError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        match validate_tokens_in_header(req) {
            Ok(c) => ready(Ok(AuthenticatedUserClaims(c))),
            Err(e) => ready(Err(e)),
        }
    }
}
