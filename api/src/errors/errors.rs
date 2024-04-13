use actix_web::error::ResponseError;
use actix_web::HttpResponse;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CustomError {
    #[error("Failed to insert country")]
    InsertCountryFailed,
}

impl ResponseError for CustomError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CustomError::InsertCountryFailed => {
                HttpResponse::InternalServerError().body("Failed to insert country")
            }
        }
    }
}
