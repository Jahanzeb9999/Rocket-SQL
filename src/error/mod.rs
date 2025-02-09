use thiserror::Error;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use serde_json::json;

#[error(Error, Debug)]
pub enum ApiError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Bad request: {0}")]
    BadRequest(String),
}

impl From<ApiError> for status::Custom<Json<serde_json::Value>> {
    fn from(error: ApiError) -> Self {
        let (status, message) = match error {
            ApiError::Database(_) => (Status::InternalServerError, "Internal server error"),
            ApiError::NotFound(msg) => (Status::NotFound, &msg),
            ApiError::BadRequest(msg) => (Status::BadRequest, &msg),
        };

        status::Custom(
            status,
            Json(json!({
                "error": message
            }))
        )
        
    }
    
}