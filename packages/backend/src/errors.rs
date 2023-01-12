use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Error struct used to represent most catchable errors in the application
#[derive(Debug, Display, Error)]
pub enum HttpError {
    #[display(fmt = "input field validation failed")]
    InputValidationError,
    #[display(fmt = "session timed out")]
    Unauthorized,
    #[display(fmt = "invalid swagger api key")]
    InvalidSwaggerAPIKey,
    #[display(fmt = "internal server error")]
    InternalServerError,
    #[display(fmt = "user not found")]
    UserNotFound,
    #[display(fmt = "password is incorrect")]
    IncorrectPassword,
}

/// Struct for formatting error into beautified json
#[derive(Serialize, Deserialize, ToSchema)]
pub struct FormattedErrorResponse {
    pub status_code: u16,
    pub error: String,
    pub message: String,
}

impl HttpError {
    fn name(&self) -> String {
        match self {
            HttpError::InputValidationError => "input validation error".to_string(),
            HttpError::Unauthorized => "unauthorized".to_string(),
            HttpError::InvalidSwaggerAPIKey => "invalid swagger api key".to_string(),
            HttpError::InternalServerError => "server error".to_string(),
            HttpError::UserNotFound => "user not found".to_string(),
            HttpError::IncorrectPassword => "incorrect password".to_string(),
        }
    }

    /// Creates a struct that implements `serde::Serialize` to be used with `json!` macro for
    /// creating examples for `swagger-ui`
    pub fn get_error_struct(self) -> FormattedErrorResponse {
        FormattedErrorResponse {
            status_code: self.match_status_code().as_u16(),
            error: self.name(),
            message: self.to_string(),
        }
    }

    fn match_status_code(&self) -> StatusCode {
        match *self {
            HttpError::InputValidationError => StatusCode::BAD_REQUEST,
            HttpError::Unauthorized => StatusCode::UNAUTHORIZED,
            HttpError::InvalidSwaggerAPIKey => StatusCode::UNAUTHORIZED,
            HttpError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            HttpError::UserNotFound => StatusCode::NOT_FOUND,
            HttpError::IncorrectPassword => StatusCode::BAD_REQUEST,
        }
    }
}

impl actix_web::error::ResponseError for HttpError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        let response = FormattedErrorResponse {
            status_code: self.status_code().as_u16(),
            error: self.name(),
            message: self.to_string(),
        };

        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(response)
    }

    fn status_code(&self) -> StatusCode {
        self.match_status_code()
    }
}