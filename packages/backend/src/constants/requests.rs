use std::{fmt::Display, str::FromStr};

use serde::{de, Deserialize, Deserializer};
use utoipa::IntoParams;

use crate::errors::HttpError;

/// This struct has to be marked unused because it is just a template for access token and refresh
/// token in the header. You would notice similar struct called [AuthenticatedClaims](crate::extractors::AuthenticatedClaims). The fact is
/// I cannot `#[derive(Deserialzie)]` on that struct type. It is needed to make all fields in
/// `kebab-case`.
#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Header)]
#[serde(rename_all = "kebab-case")]
#[allow(unused)]
pub struct AuthenticationHeaders {
    /// User's access token
    x_access_token: String,
    /// User's refresh token
    x_refresh_token: String,
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