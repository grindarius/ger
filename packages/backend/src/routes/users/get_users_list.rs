use actix_web::{web, HttpResponse};
use serde::Deserialize;
use ts_rs::TS;
use utoipa::{IntoParams, ToSchema};

use crate::{
    constants::{DEFAULT_PAGE, DEFAULT_PAGE_SIZE},
    errors::HttpError,
    shared_app_data::SharedAppData,
};

#[derive(Deserialize, IntoParams, ToSchema, TS)]
#[ts(export)]
#[into_params(parameter_in = Query)]
pub struct GetUsersListRequestQueries {
    #[param(minimum = 1, default = json!(DEFAULT_PAGE))]
    #[serde(deserialize_with = "crate::constants::requests::deserialize_page")]
    #[ts(optional)]
    pub page: Option<i32>,
    #[param(minimum = 1, maximum = 100, default = json!(DEFAULT_PAGE_SIZE))]
    #[serde(deserialize_with = "crate::constants::requests::deserialize_page_size")]
    #[ts(optional)]
    pub page_size: Option<i32>,
}

#[utoipa::path(
    get,
    path = "/users",
    tag = "users",
    operation_id = "get_users_list",
    params(GetUsersListRequestQueries)
)]
pub async fn handler(
    query: web::Query<GetUsersListRequestQueries>,
    data: web::Data<SharedAppData>,
) -> Result<HttpResponse, HttpError> {
    Ok(HttpResponse::Ok().finish())
}
