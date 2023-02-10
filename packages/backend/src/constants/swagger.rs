use serde::Deserialize;

/// This struct has to be marked unused because it is just a template for access token and refresh
/// token in the header. You would notice similar struct called [AuthenticatedClaims](crate::extractors::AuthenticatedClaims). The fact is
/// I cannot derive deserialize on that struct type. It is needed to make all fields in
/// `kebab-case`.
#[derive(Deserialize, utoipa::IntoParams)]
#[into_params(parameter_in = Header)]
#[serde(rename_all = "kebab-case")]
#[allow(unused)]
pub struct AuthenticationHeaders {
    /// User's access token
    x_access_token: String,
    /// User's refresh token
    x_refresh_token: String,
}
