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
    #[display(fmt = "input validation error on field {}", field)]
    InputValidationError { field: String },
    #[display(fmt = "an internal server error occured, please try again later")]
    InternalServerError,
    #[display(fmt = "an error occured while hashing your password, please try again")]
    PasswordHashError,
    #[display(fmt = "{} is an empty field", field)]
    InputFieldEmptyError { field: String },
    #[display(fmt = "{} not found", query)]
    EntityNotFoundError { query: String },
    #[display(fmt = "database row deserialization error")]
    RowDeserializeError,
    #[display(fmt = "cannot parse password hash from the database")]
    PasswordHashParseError,
    #[display(fmt = "password does not match")]
    PasswordIncorrectError,
    #[display(fmt = "unauthorized, please refresh your tokens")]
    Unauthorized,
    #[display(fmt = "invalid swagger api key")]
    InvalidSwaggerAPIKey,
    #[display(fmt = "invalid username format")]
    InvalidUsernameFormat,
    #[display(fmt = "invalid email format")]
    InvalidEmailFormat,
    #[display(fmt = "\"{}\" exists", name)]
    Exists { name: String },
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
            HttpError::InputValidationError { .. } => "validation error".to_string(),
            HttpError::InternalServerError => "internal server error".to_string(),
            HttpError::PasswordHashError => "password hash error".to_string(),
            HttpError::InputFieldEmptyError { .. } => "field empty error".to_string(),
            HttpError::EntityNotFoundError { .. } => "not found".to_string(),
            HttpError::RowDeserializeError => "row deserialize error".to_string(),
            HttpError::PasswordHashParseError => "password hash parse error".to_string(),
            HttpError::PasswordIncorrectError => "password incorrect".to_string(),
            HttpError::Unauthorized => "unauthorized".to_string(),
            HttpError::InvalidSwaggerAPIKey => "unauthorized".to_string(),
            HttpError::InvalidUsernameFormat => "invalid username format".to_string(),
            HttpError::InvalidEmailFormat => "invalid email format".to_string(),
            HttpError::Exists { .. } => "username exists".to_string(),
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
            HttpError::InputValidationError { .. } => StatusCode::BAD_REQUEST,
            HttpError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            HttpError::PasswordHashError => StatusCode::INTERNAL_SERVER_ERROR,
            HttpError::InputFieldEmptyError { .. } => StatusCode::BAD_REQUEST,
            HttpError::EntityNotFoundError { .. } => StatusCode::NOT_FOUND,
            HttpError::RowDeserializeError => StatusCode::INTERNAL_SERVER_ERROR,
            HttpError::PasswordHashParseError => StatusCode::INTERNAL_SERVER_ERROR,
            HttpError::PasswordIncorrectError => StatusCode::BAD_REQUEST,
            HttpError::Unauthorized => StatusCode::UNAUTHORIZED,
            HttpError::InvalidSwaggerAPIKey => StatusCode::UNAUTHORIZED,
            HttpError::InvalidUsernameFormat => StatusCode::BAD_REQUEST,
            HttpError::InvalidEmailFormat => StatusCode::BAD_REQUEST,
            HttpError::Exists { .. } => StatusCode::BAD_REQUEST,
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
