use actix_web::{web, HttpResponse};
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

use crate::{errors::HttpError, shared_app_data::SharedAppData};

#[derive(Deserialize, IntoParams, ToSchema)]
#[into_params(parameter_in = Query)]
pub struct GetUsersListQueries {
    #[serde(default, deserialize_with = "crate::constants::empty_string_as_none")]
    #[param(minimum = 0, default = json!(1))]
    pub page: Option<i32>,
    #[serde(default, deserialize_with = "crate::constants::empty_string_as_none")]
    #[param(minimum = 1, default = json!(crate::constants::DEFAULT_PAGE_SIZE))]
    pub page_size: Option<i32>,
}

#[utoipa::path(
    get,
    path = "/users",
    tag = "users",
    operation_id = "get_users_list",
    params(GetUsersListQueries)
)]
pub async fn handler(
    queries: web::Query<GetUsersListQueries>,
    data: web::Data<SharedAppData>,
) -> Result<HttpResponse, HttpError> {
    Ok(HttpResponse::Ok().finish())
}
