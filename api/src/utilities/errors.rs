use actix_web::error::ResponseError;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use sea_orm::DbErr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CustomErrors {
    #[error("Database error: {0}")]
    DatabaseError(DbErr),
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("Authentication error: {0}")]
    AuthenticationError(String),
    #[error("Resource not found: {0}")]
    NotFoundError(String),
}

impl ResponseError for CustomErrors {
    fn status_code(&self) -> StatusCode {
        match self {
            CustomErrors::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            CustomErrors::ValidationError(_) => StatusCode::BAD_REQUEST,
            CustomErrors::AuthenticationError(_) => StatusCode::UNAUTHORIZED,
            CustomErrors::NotFoundError(_) => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        match self {
            CustomErrors::DatabaseError(db_error) => eprintln!("Database error occurred: {}", db_error),
            CustomErrors::ValidationError(err) => eprintln!("Validation error occurred: {}", err),
            CustomErrors::AuthenticationError(err) => eprintln!("Authentication error occurred: {}", err),
            CustomErrors::NotFoundError(err) => eprintln!("Resource not found error occurred: {}", err),
        }
        HttpResponse::build(self.status_code()).body(self.to_string())
    }
}
