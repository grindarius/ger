use actix_web::{web, HttpResponse};
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

use crate::{errors::HttpError, shared_app_data::SharedAppData};

#[derive(Deserialize, IntoParams, ToSchema)]
#[into_params(parameter_in = Query)]
pub struct GetUsersListRequestQueries {
    #[param(minimum = 1, default = json!(crate::constants::DEFAULT_PAGE))]
    #[serde(
        default,
        deserialize_with = "crate::constants::requests::empty_string_as_none"
    )]
    pub page: Option<i32>,
    #[param(minimum = 1, default = json!(crate::constants::DEFAULT_PAGE_SIZE))]
    #[serde(
        default,
        deserialize_with = "crate::constants::requests::empty_string_as_none"
    )]
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
