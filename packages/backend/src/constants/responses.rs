use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct GetServerInformationResponse {
    contributors: Vec<String>,
    contact: String,
}

impl Default for GetServerInformationResponse {
    fn default() -> Self {
        Self {
            contributors: vec!["Bhattarpong Somwong".to_string()],
            contact: "numbbutt34685@gmail.com".to_string(),
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct DefaultSuccessResponse {
    message: String,
}

impl Default for DefaultSuccessResponse {
    fn default() -> Self {
        Self {
            message: "completed".to_string(),
        }
    }
}

impl DefaultSuccessResponse {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}