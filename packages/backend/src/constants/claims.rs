use serde::{Deserialize, Serialize};

use crate::{constants::JWT_TOKEN_AUDIENCE_NAME, database::Role, errors::HttpError};

#[derive(Serialize, Deserialize, PartialEq)]
pub struct AccessTokenClaims {
    pub aud: String,
    pub exp: usize,
    pub iat: usize,
    pub uid: String,
    pub sid: String,
    pub rle: Role,
}

impl AccessTokenClaims {
    /// Creates new access token claims with required parameters.
    ///
    /// # Panics
    ///
    /// The instantiation could fail from converting `time::offsetDateTime` wrongly
    pub fn new(
        user_id: String,
        user_role: Role,
        session_id: String,
        expires_timestamp: usize,
    ) -> Result<Self, HttpError> {
        Ok(Self {
            aud: JWT_TOKEN_AUDIENCE_NAME.to_string(),
            exp: expires_timestamp,
            iat: usize::try_from(time::OffsetDateTime::now_utc().unix_timestamp()).map_err(
                |_| HttpError::InternalServerError {
                    cause: "cannot convert timestamp from type i64 to usize".to_string(),
                },
            )?,
            uid: user_id,
            sid: session_id,
            rle: user_role,
        })
    }
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct RefreshTokenClaims {
    pub aud: String,
    pub exp: usize,
    pub iat: usize,
    pub uid: String,
    pub sid: String,
}

impl RefreshTokenClaims {
    /// Creates new refresh token claims with required parameters.
    ///
    /// # Panics
    ///
    /// The instantiation could fail from converting `time::offsetDateTime` wrongly
    pub fn new(
        user_id: String,
        session_id: String,
        expires_timestamp: usize,
    ) -> Result<Self, HttpError> {
        Ok(Self {
            aud: JWT_TOKEN_AUDIENCE_NAME.to_string(),
            exp: expires_timestamp,
            iat: usize::try_from(time::OffsetDateTime::now_utc().unix_timestamp()).map_err(
                |_| HttpError::InternalServerError {
                    cause: "error converting timestamp from i64 to usize".to_string(),
                },
            )?,
            uid: user_id,
            sid: session_id,
        })
    }
}

/// Get utc expires time from current time.
pub fn get_expires_timestamp(valid_minutes: u32) -> Result<usize, HttpError> {
    let current_time =
        time::OffsetDateTime::now_utc() + time::Duration::minutes(valid_minutes as i64);

    return usize::try_from(current_time.unix_timestamp()).map_err(|_| {
        HttpError::InternalServerError {
            cause: "cannot convert timestamp from type i64 to usize".to_string(),
        }
    });
}
