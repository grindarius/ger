use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};
use serde_variant::UnsupportedType;
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
    #[display(fmt = "{}", cause)]
    InternalServerError { cause: String },
    #[display(fmt = "user not found")]
    UserNotFound,
    #[display(fmt = "post not found")]
    PostNotFound,
    #[display(fmt = "category not found")]
    CategoryNotFound,
    #[display(fmt = "password is incorrect")]
    IncorrectPassword,
    #[display(fmt = "incoming data is empty")]
    NoData,
    #[display(fmt = "you do not have to role to access this content")]
    Forbidden,
    #[display(fmt = "invalid authentication credentials")]
    InvalidAuthenticationCredentials,
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
            HttpError::InternalServerError { .. } => "internal server error".to_string(),
            HttpError::UserNotFound => "user not found".to_string(),
            HttpError::PostNotFound => "post not found".to_string(),
            HttpError::CategoryNotFound => "category not found".to_string(),
            HttpError::IncorrectPassword => "incorrect password".to_string(),
            HttpError::NoData => "no data".to_string(),
            HttpError::Forbidden => "forbidden".to_string(),
            HttpError::InvalidAuthenticationCredentials => {
                "invalid authentication credentials".to_string()
            }
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
            HttpError::InternalServerError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            HttpError::UserNotFound => StatusCode::NOT_FOUND,
            HttpError::PostNotFound => StatusCode::NOT_FOUND,
            HttpError::CategoryNotFound => StatusCode::NOT_FOUND,
            HttpError::IncorrectPassword => StatusCode::BAD_REQUEST,
            HttpError::NoData => StatusCode::BAD_REQUEST,
            HttpError::Forbidden => StatusCode::FORBIDDEN,
            HttpError::InvalidAuthenticationCredentials => StatusCode::BAD_REQUEST,
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

impl From<deadpool_postgres::PoolError> for HttpError {
    fn from(error: deadpool_postgres::PoolError) -> Self {
        HttpError::InternalServerError {
            cause: error.to_string(),
        }
    }
}

impl From<tokio_postgres::Error> for HttpError {
    fn from(error: tokio_postgres::Error) -> Self {
        HttpError::InternalServerError {
            cause: error.to_string(),
        }
    }
}

impl From<argon2::Error> for HttpError {
    fn from(error: argon2::Error) -> Self {
        HttpError::InternalServerError {
            cause: error.to_string(),
        }
    }
}

impl From<argon2::password_hash::Error> for HttpError {
    fn from(error: argon2::password_hash::Error) -> Self {
        HttpError::InternalServerError {
            cause: error.to_string(),
        }
    }
}

impl From<anyhow::Error> for HttpError {
    fn from(error: anyhow::Error) -> Self {
        HttpError::InternalServerError {
            cause: error.to_string(),
        }
    }
}

impl From<jsonwebtoken::errors::Error> for HttpError {
    fn from(error: jsonwebtoken::errors::Error) -> Self {
        HttpError::InternalServerError {
            cause: error.to_string(),
        }
    }
}

impl From<UnsupportedType> for HttpError {
    fn from(error: UnsupportedType) -> Self {
        HttpError::InternalServerError {
            cause: error.to_string(),
        }
    }
}
