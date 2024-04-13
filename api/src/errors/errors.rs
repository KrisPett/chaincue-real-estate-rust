use actix_web::error::ResponseError;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use sea_orm::DbErr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CustomError {
    #[error("Database error: {0}")]
    DatabaseError(DbErr),
}

impl ResponseError for CustomError {
    fn status_code(&self) -> StatusCode {
        match self {
            CustomError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    fn error_response(&self) -> HttpResponse {
        match self {
            CustomError::DatabaseError(db_error) => {
                eprintln!("Database error occurred: {}", db_error);
            }
        }
        HttpResponse::build(self.status_code()).body(self.to_string())
    }
}
