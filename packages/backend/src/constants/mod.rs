use std::{fmt::Display, str::FromStr};

use argon2::{Algorithm as Argon2Algorithm, Argon2, Params, Version};
use comrak::ComrakOptions;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use lazy_static::lazy_static;
use serde::{de, Deserialize, Deserializer};

use crate::errors::HttpError;

pub mod claims;
pub mod responses;
pub mod swagger;

lazy_static! {
    pub static ref HEADER: Header = Header::new(Algorithm::RS256);
    pub static ref VALIDATION: Validation = Validation::new(Algorithm::RS256);
    pub static ref ACCESS_TOKEN_ENCODING_KEY: EncodingKey = {
        EncodingKey::from_rsa_pem(include_bytes!(
            "../../jsonwebtoken/access_token_private_key.pem"
        ))
        .expect("cannot create access token private key in constants.rs")
    };
    pub static ref REFRESH_TOKEN_ENCODING_KEY: EncodingKey = {
        EncodingKey::from_rsa_pem(include_bytes!(
            "../../jsonwebtoken/refresh_token_private_key.pem"
        ))
        .expect("cannot create refresh token private key in constants.rs")
    };
    pub static ref ACCESS_TOKEN_DECODING_KEY: DecodingKey = {
        DecodingKey::from_rsa_pem(include_bytes!(
            "../../jsonwebtoken/access_token_public_key.pem"
        ))
        .expect("cannot create access token public key in constants.rs")
    };
    pub static ref REFRESH_TOKEN_DECODING_KEY: DecodingKey = {
        DecodingKey::from_rsa_pem(include_bytes!(
            "../../jsonwebtoken/refresh_token_public_key.pem"
        ))
        .expect("cannot create refresh token public key in constants.rs")
    };
    pub static ref SWAGGER_API_KEY_NAME: String =
        dotenvy::var("GER_SWAGGER_API_KEY_NAME").expect("cannot load swagger api key name");
    pub static ref SWAGGER_API_KEY: String =
        dotenvy::var("GER_SWAGGER_API_KEY").expect("cannot load swagger api key");
    pub static ref JWT_TOKEN_AUDIENCE_NAME: String = "ger.com".to_string();
    pub static ref ARGON2_PEPPER_STRING: String =
        dotenvy::var("GER_ARGON2_PEPPER").expect("cannot load argon2 pepper string");
    pub static ref COMRAK_OPTIONS: ComrakOptions = {
        let mut options = ComrakOptions::default();
        options.extension.strikethrough = true;
        options.extension.table = true;
        options.extension.autolink = true;
        options.extension.tasklist = true;

        options
    };
}

pub fn create_argon2_context<'key>(
    pepper: &'key str,
) -> Result<argon2::Argon2<'key>, argon2::Error> {
    let context: Argon2 = Argon2::new_with_secret(
        pepper.as_bytes(),
        Argon2Algorithm::Argon2id,
        Version::V0x13,
        Params::new(20000u32, 3u32, 3u32, Some(64usize))?,
    )?;

    Ok(context)
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

/// Deserialize a given string option as `None` when a given string is an empty string.
///
/// This is a workaround from [this issue](https://github.com/actix/actix-web/issues/1815)
///
/// Solution taken from [serde#1425](https://github.com/serde-rs/serde/issues/1425#issuecomment-439728211)
pub fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: Display,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
    }
}

/// Sql number range used to query data as in parts
pub struct SqlRange {
    pub limit: i32,
    pub offset: i32,
}

impl SqlRange {
    /// Create `limit` and `offset` values used to query data from the database.
    ///
    /// # Panics
    /// returns error when either `page` or `page_size` is less than zero.
    pub fn from_page(page: i32, page_size: i32) -> Result<Self, HttpError> {
        if !page.is_positive() || !page_size.is_positive() {
            return Err(HttpError::InputValidationError);
        }

        Ok(Self {
            limit: page_size,
            offset: (page * page_size) - page_size,
        })
    }
}

/// difference between AD (Anno domini) year and BE (Bhuddist era) year.
pub const AD_BE_YEAR_DIFFERENCE: u32 = 543;

/// application name
pub const APP_NAME: &'static str = "ger";

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

/// Default page when page is missing
pub const DEFAULT_PAGE: i32 = 1;

/// Default page size when page size is missing
pub const DEFAULT_PAGE_SIZE: i32 = 10;

/// Default trending window for
/// [get_trending_posts_list](crate::routes::forum::get_trending_posts_list::handler)
pub const DEFAULT_TRENDING_WINDOW: i32 = 24;
