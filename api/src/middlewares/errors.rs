use std::io;
use std::io::ErrorKind;

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

impl From<CustomErrors> for io::Error {
    fn from(error: CustomErrors) -> Self {
        match error {
            CustomErrors::DatabaseError(err) => {
                log::error!("Database error occurred: {}", err);
                io::Error::new(ErrorKind::Other, format!("Database error: {}", err))
            }
            CustomErrors::ValidationError(err) => {
                log::error!("Validation error occurred: {}", err);
                io::Error::new(ErrorKind::InvalidInput, format!("Validation error: {}", err))
            }
            CustomErrors::AuthenticationError(err) => {
                log::error!("Authentication error occurred: {}", err);
                io::Error::new(ErrorKind::PermissionDenied, format!("Authentication error: {}", err))
            }
            CustomErrors::NotFoundError(err) => {
                log::error!("Resource not found error occurred: {}", err);
                io::Error::new(ErrorKind::NotFound, format!("Resource not found: {}", err))
            }
        }
    }
}
