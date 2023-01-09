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
    pub static ref ID_LENGTH: u8 = 32;
    pub static ref FILE_NAME_LENGTH: u8 = 48;
    pub static ref JWT_TOKEN_AUDIENCE_NAME: String = "ger.com".to_string();
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

#[derive(Serialize, Deserialize)]
pub struct RefreshTokenClaims {
    aud: String,
    exp: usize,
    iat: usize,
    uid: String,
    sid: String,
}

impl AccessTokenClaims {
    /// Creates new access token claims with required parameters. The instantiation could fail from
    pub fn try_new(
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
