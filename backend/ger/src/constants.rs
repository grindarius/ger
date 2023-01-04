use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

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
    pub static ref SWAGGER_API_KEY_NAME: String = dotenvy::var("GER_SWAGGER_API_KEY_NAME")
        .expect("cannot load swagger api key name");
    pub static ref SWAGGER_API_KEY: String = dotenvy::var("GER_SWAGGER_API_KEY")
        .expect("cannot load swagger api key");
    pub static ref ID_LENGTH: u8 = 32;
    pub static ref FILE_NAME_LENGTH: u8 = 48;
}

#[derive(ger_from_row::FromRow, Serialize, Deserialize)]
pub enum Role {
    Student,
    Professor,
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

#[derive(Serialize)]
pub struct GetServerInformationResponse {
    contributors: Vec<String>,
    contact: String,
}

impl Default for GetServerInformationResponse {
    fn default() -> Self {
        Self {
            contributors: vec!["Bhattarpaong Somwong".to_string()],
            contact: "numbbutt34685@gmail.com".to_string(),
        }
    }
}
