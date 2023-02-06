use serde::Deserialize;

#[derive(Deserialize, utoipa::IntoParams)]
#[into_params(parameter_in = Header)]
#[serde(rename_all = "kebab-case")]
pub struct AuthenticationHeaders {
    /// User's access token
    x_access_token: String,
    /// User's refresh token
    x_refresh_token: String,
}
