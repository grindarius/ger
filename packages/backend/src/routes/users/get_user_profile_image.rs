use actix_web::{web, HttpResponse};
use serde::Deserialize;
use utoipa::IntoParams;

use crate::{
    constants::requests::AuthenticationHeaders, errors::HttpError,
    extractors::users::AuthenticatedUserClaims,
};

#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Path)]
pub struct GetUserProfileImageParams {
    user_id: String,
}

#[utoipa::path(
    get,
    path = "/users/{user_id}/profile-image",
    tag = "users",
    operation_id = "get_user_profile_image",
    params(AuthenticationHeaders, GetUserProfileImageParams)
)]
pub async fn handler(
    path: web::Path<GetUserProfileImageParams>,
    _claims: AuthenticatedUserClaims,
) -> Result<HttpResponse, HttpError> {
    Ok(HttpResponse::Ok().finish())
}

/// Generates a unique pastel color from a given username
fn username_to_hsl_color(name: &str) -> u32 {
    let mut hash: u32 = 0;

    for c in name.chars() {
        hash = (c as u32) + ((hash << 5) - hash);
    }

    let hash = hash % 360;
    hash
}
