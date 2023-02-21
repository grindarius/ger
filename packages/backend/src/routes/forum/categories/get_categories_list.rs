use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use serde_json::json;
use ts_rs::TS;
use utoipa::{IntoParams, ToSchema};

use crate::{constants::DEFAULT_PAGE, constants::DEFAULT_PAGE_SIZE, errors::HttpError};

#[derive(Deserialize, ToSchema, IntoParams, TS)]
#[into_params(parameter_in = Query)]
#[ts(export)]
pub struct GetCategoriesListRequestQueries {
    #[param(minimum = 1, default = json!(DEFAULT_PAGE))]
    #[serde(default, deserialize_with = "crate::constants::empty_string_as_none")]
    #[ts(optional)]
    pub page: Option<i32>,
    #[param(minimum = 1, default = json!(DEFAULT_PAGE_SIZE))]
    #[serde(default, deserialize_with = "crate::constants::empty_string_as_none")]
    #[ts(optional)]
    pub page_size: Option<i32>,
}

#[derive(Serialize, ToSchema, TS)]
#[ts(export)]
pub struct GetCategoriesListResponseBody {
    categories: Vec<GetCategoriesListResponseBodyInner>,
}

#[derive(Serialize, ToSchema, TS)]
#[ts(export)]
pub struct GetCategoriesListResponseBodyInner {
    id: String,
    representative_id: String,
    posts_count: i64,
    latest_post_id: String,
    latest_post_name: String,
    latest_post_user_id: String,
    #[serde(with = "time::serde::rfc3339")]
    #[ts(type = "string")]
    latest_post_created_timestamp: time::OffsetDateTime,
}

#[utoipa::path(
    get,
    path = "/forum/categories",
    tag = "forum",
    operation_id = "get_categories_list",
    params(GetCategoriesListRequestQueries),
    responses(
        (
            status = 200,
            description = "successfully get list of forum categories",
            body = GetCategoriesListResponseBody,
            example = json!({ "categories": [] })
        ),
    )
)]
pub async fn handler() -> Result<HttpResponse, HttpError> {
    Ok(HttpResponse::Ok().json(json!({ "categories": [] })))
}
