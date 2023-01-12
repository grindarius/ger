use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::errors::HttpError;

lazy_static! {
    pub static ref HEADER: Header = Header::new(Algorithm::RS256);
    pub static ref VALIDATION: Validation = Validation::new(Algorithm::RS256);
    pub static ref ACCESS_TOKEN_ENCODING_KEY: EncodingKey = {
        let access_token_private_key = dotenvy::var("GER_ACCESS_TOKEN_PRIVATE_KEY")
            .expect("missing access token private key in constants.rs");
        EncodingKey::from_rsa_pem(&access_token_private_key.as_bytes())
            .expect("cannot create access token private key in constants.rs")
    };
    pub static ref REFRESH_TOKEN_ENCODING_KEY: EncodingKey = {
        let refresh_token_private_key = dotenvy::var("GER_REFRESH_TOKEN_PRIVATE_KEY")
            .expect("missing refresh token private key in constants.rs");
        EncodingKey::from_rsa_pem(&refresh_token_private_key.as_bytes())
            .expect("cannot create refresh token private key in constants.rs")
    };
    pub static ref ACCESS_TOKEN_DECODING_KEY: DecodingKey = {
        let access_token_public_key = dotenvy::var("GER_ACCESS_TOKEN_PUBLIC_KEY")
            .expect("cannot create access token public key in constants.rs");
        DecodingKey::from_rsa_pem(&access_token_public_key.as_bytes())
            .expect("cannot create access token public key in constants.rs")
    };
    pub static ref REFRESH_TOKEN_DECODING_KEY: DecodingKey = {
        let refresh_token_public_key = dotenvy::var("GER_REFRESH_TOKEN_PUBLIC_KEY")
            .expect("cannot create refresh token public key in constants.rs");
        DecodingKey::from_rsa_pem(&refresh_token_public_key.as_bytes())
            .expect("cannot create refresh token public key in constants.rs")
    };
    pub static ref SWAGGER_API_KEY_NAME: String =
        dotenvy::var("GER_SWAGGER_API_KEY_NAME").expect("cannot load swagger api key name");
    pub static ref SWAGGER_API_KEY: String =
        dotenvy::var("GER_SWAGGER_API_KEY").expect("cannot load swagger api key");
    pub static ref JWT_TOKEN_AUDIENCE_NAME: String = "ger.com".to_string();
}

/// Length of id used in most primary keys.
pub const ID_LENGTH: usize = 32;

/// Length of id used in file names when user created.
pub const FILE_NAME_LENGTH: u8 = 48;

/// How long an access token can be valid for in minutes.
pub const ACCESS_TOKEN_VALID_TIME_LENGTH: u32 = 15;

/// How long a refresh token can be valid for in minutes.
pub const REFRESH_TOKEN_VALID_TIME_LENGTH: u32 = 60 * 24 * 7;

/// The name of header that carries access token
pub const ACCESS_TOKEN_HEADER_NAME: &'static str = "x-access-token";

/// The name of header that carries refresh token
pub const REFRESH_TOKEN_HEADER_NAME: &'static str = "x-refresh-token";

/// Get utc expires time from current time.
pub fn get_expires_timestamp(valid_minutes: u32) -> Result<usize, HttpError> {
    let current_time =
        time::OffsetDateTime::now_utc() + time::Duration::minutes(valid_minutes as i64);

    return usize::try_from(current_time.unix_timestamp())
        .map_err(|_| HttpError::InternalServerError);
}

/// Possible roles of any users in the server
#[derive(ger_from_row::FromRow, Serialize, Deserialize)]
pub enum Role {
    /// Student role for students, the student's account cannot be created by the students theirselves.
    Student,
    /// Professor role for professors, the professor's account cannot be created by the professor theirselves.
    Professor,
    /// Admins. simple as that.
    Admin,
}

#[derive(Serialize, Deserialize)]
pub struct AccessTokenClaims {
    aud: String,
    exp: usize,
    iat: usize,
    uid: String,
    sid: String,
    rle: Role,
}

impl AccessTokenClaims {
    /// Creates new access token claims with required parameters.
    ///
    /// # Panics
    ///
    /// The instantiation could fail from converting offsetdatetime wrongly
    pub fn new(
        user_id: String,
        user_role: Role,
        session_id: String,
        expires_timestamp: usize,
    ) -> Result<Self, HttpError> {
        Ok(Self {
            aud: JWT_TOKEN_AUDIENCE_NAME.to_string(),
            exp: expires_timestamp,
            iat: usize::try_from(time::OffsetDateTime::now_utc().unix_timestamp())
                .map_err(|_| HttpError::InternalServerError)?,
            uid: user_id,
            sid: session_id,
            rle: user_role,
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct RefreshTokenClaims {
    aud: String,
    exp: usize,
    iat: usize,
    uid: String,
    sid: String,
}

impl RefreshTokenClaims {
    pub fn new(
        user_id: String,
        session_id: String,
        expires_timestamp: usize,
    ) -> Result<Self, HttpError> {
        Ok(Self {
            aud: JWT_TOKEN_AUDIENCE_NAME.to_string(),
            exp: expires_timestamp,
            iat: usize::try_from(time::OffsetDateTime::now_utc().unix_timestamp())
                .map_err(|_| HttpError::InternalServerError)?,
            uid: user_id,
            sid: session_id,
        })
    }
}

#[derive(Serialize, ToSchema)]
pub struct GetServerInformationResponse {
    contributors: Vec<String>,
    contact: String,
}

impl Default for GetServerInformationResponse {
    fn default() -> Self {
        Self {
            contributors: vec!["Bhattarpong Somwong".to_string()],
            contact: "numbbutt34685@gmail.com".to_string(),
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct DefaultSuccessResponse {
    message: String,
}

impl Default for DefaultSuccessResponse {
    fn default() -> Self {
        Self {
            message: "completed".to_string(),
        }
    }
}

impl DefaultSuccessResponse {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}
