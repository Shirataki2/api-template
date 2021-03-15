use thiserror::Error;
use serde::Serialize;
use actix_web::{HttpResponse, http::StatusCode, ResponseError};

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Requested Resource is Not Found")]
    NotFound,
    #[error("Error occurred during authentication.")]
    AuthorizationServerError,
    #[error("Unexpected error has occurred.")]
    InternalServerError,
    #[error("Failed to read tokens from server.")]
    TokenRegistrationError,
    #[error("{0}")]
    Unauthorized(String),
    #[error("An error has occurred while communicating with Discord.")]
    SerenityError(#[from] serenity::Error)
}

impl AppError {
    pub fn name(&self) -> String {
        let err = match *self {
            Self::NotFound => "Not Found",
            Self::AuthorizationServerError => "Internal Server Error",
            Self::InternalServerError => "Internal Server Error",
            Self::TokenRegistrationError => "Internal Server Error",
            Self::Unauthorized(_) => "Unauthorized",
            Self::SerenityError(_) => "Service Temporarily Unavailable",
        };
        String::from(err)
    }
}

#[derive(Serialize, Debug)]
struct ErrorResponse {
    code: u16,
    error: String,
    message: String,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match *self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::AuthorizationServerError => StatusCode::INTERNAL_SERVER_ERROR,
            Self::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            Self::TokenRegistrationError => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Self::SerenityError(_) => StatusCode::SERVICE_UNAVAILABLE,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let resp = ErrorResponse {
            code: status_code.as_u16(),
            message: self.to_string(),
            error: self.name(),
        };
        HttpResponse::build(status_code).json(resp)
    }
}

impl From<actix_web::Error> for AppError {
    fn from(err: actix_web::Error) -> AppError {
        error!("{:#?}", err);
        AppError::InternalServerError
    }
}

