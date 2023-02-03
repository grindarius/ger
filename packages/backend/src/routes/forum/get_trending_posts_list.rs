use std::{fmt::Display, str::FromStr};

use actix_web::{web, HttpResponse};
use serde::{de, Deserialize, Deserializer};
use utoipa::{IntoParams, ToSchema};

use crate::{errors::HttpError, extractors::users::AuthenticatedUserClaims};

#[derive(Deserialize, ToSchema, IntoParams)]
#[into_params(style = Form, parameter_in = Query)]
pub struct GetTrendingPostsListRequestQueries {
    /// How big of a window to check for the trending posts. like "trending in the last 24
    /// hours"
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub hours: Option<u32>,
    /// How much of a post to query for.
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub limit: Option<u32>,
    /// How much of a post to skip as a page change.
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub skip: Option<u32>,
}

/// Gets trending posts list with a given limit and a time window.
#[utoipa::path(get, path = "/forum/trending")]
pub async fn handler(
    query: web::Query<GetTrendingPostsListRequestQueries>,
    _claims: AuthenticatedUserClaims,
) -> Result<HttpResponse, HttpError> {
    Ok(HttpResponse::Ok().finish())
}

/// Deserialize a given string option as `None` when a given string is an empty string.
///
/// This is a workaround from [this issue](https://github.com/actix/actix-web/issues/1815)
///
/// Solution taken from [serde#1425](https://github.com/serde-rs/serde/issues/1425#issuecomment-439728211)
fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
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
