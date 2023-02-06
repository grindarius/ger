use std::{fmt::Display, str::FromStr};

use actix_web::{web, HttpResponse};
use serde::{de, Deserialize, Deserializer};
use utoipa::{IntoParams, ToSchema};

use crate::{
    constants::swagger::AuthenticationHeaders, errors::HttpError,
    extractors::users::AuthenticatedUserClaims,
};

#[derive(Deserialize, ToSchema, IntoParams)]
#[into_params(style = Form, parameter_in = Query)]
pub struct GetTrendingPostsListRequestQueries {
    /// How big of a window to check for the trending posts. like "trending in the last 24
    /// hours". default is 24
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub hours: Option<u32>,
    /// How much of a post to query for. default is 10
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub limit: Option<u32>,
    /// How much of a post to skip as a page change. default is 0
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub offset: Option<u32>,
}

/// Gets trending posts list with a given limit and a time window.
#[utoipa::path(
    get,
    path = "/forum/trending",
    tag = "forums",
    params(AuthenticationHeaders, GetTrendingPostsListRequestQueries)
)]
pub async fn handler(
    query: web::Query<GetTrendingPostsListRequestQueries>,
    _claims: AuthenticatedUserClaims,
) -> Result<HttpResponse, HttpError> {
    let hours = query.hours.unwrap_or(24);
    let limit = query.limit.unwrap_or(10);
    let offset = query.offset.unwrap_or(0);

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

#[cfg(test)]
mod tests {
    use actix_web::{http::header::ContentType, test, web::Query};

    use super::*;

    #[actix_web::test]
    async fn test_query_deserializer() {
        let request = test::TestRequest::default()
            .insert_header(ContentType::json())
            .to_http_request();

        let query = Query(GetTrendingPostsListRequestQueries {
            hours: 24,
            limit: 10,
            offset: 0,
        });

        let response = handler(query, claims).await;
    }
}
