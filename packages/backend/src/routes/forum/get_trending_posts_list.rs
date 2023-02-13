use actix_web::{web, HttpResponse};
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

use crate::{
    constants::{
        swagger::AuthenticationHeaders, DEFAULT_PAGE, DEFAULT_PAGE_SIZE, DEFAULT_TRENDING_WINDOW,
    },
    errors::HttpError,
    extractors::users::AuthenticatedUserClaims,
};

#[derive(Deserialize, ToSchema, IntoParams)]
#[into_params(style = Form, parameter_in = Query)]
pub struct GetTrendingPostsListRequestQueries {
    /// How big of a window to check for the trending posts. like "trending in the last 24
    /// hours". default is `24`.
    #[serde(default, deserialize_with = "crate::constants::empty_string_as_none")]
    #[param(minimum = 0)]
    pub hours: Option<i32>,
    /// How much of a post to query for. default is `10`.
    #[serde(default, deserialize_with = "crate::constants::empty_string_as_none")]
    #[param(minimum = 0)]
    pub page: Option<i32>,
    /// How much of a post to skip as a page change. default is `10`
    #[serde(default, deserialize_with = "crate::constants::empty_string_as_none")]
    #[param(minimum = 0)]
    pub page_size: Option<i32>,
}

/// Gets trending posts list with a given page size and time window.
#[utoipa::path(
    get,
    path = "/forum/trending",
    tag = "forum",
    operation_id = "get_trending_posts_list",
    params(AuthenticationHeaders, GetTrendingPostsListRequestQueries)
)]
pub async fn handler(
    query: web::Query<GetTrendingPostsListRequestQueries>,
    _claims: AuthenticatedUserClaims,
) -> Result<HttpResponse, HttpError> {
    let hours = query.hours.unwrap_or(DEFAULT_TRENDING_WINDOW);
    let page = query.page.unwrap_or(DEFAULT_PAGE);
    let page_size = query.page_size.unwrap_or(DEFAULT_PAGE_SIZE);

    tracing::info!(hours);
    tracing::info!(page);
    tracing::info!(page_size);

    Ok(HttpResponse::Ok().finish())
}
