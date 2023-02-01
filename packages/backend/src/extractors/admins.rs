use actix_web::{dev::Payload, FromRequest, HttpRequest};
use futures_util::future::{ready, Ready};

use crate::{database::Role, errors::HttpError};

use super::{validate_tokens_in_header, AuthenticatedClaims};

pub struct AuthenticatedAdminClaims(pub AuthenticatedClaims);

impl FromRequest for AuthenticatedAdminClaims {
    type Error = HttpError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let claims = match validate_tokens_in_header(req) {
            Ok(c) => c,
            Err(e) => return ready(Err(e)),
        };

        if claims.access_token.claims.rle != Role::Admin {
            return ready(Err(HttpError::Unauthorized));
        }

        ready(Ok(AuthenticatedAdminClaims(claims)))
    }
}
