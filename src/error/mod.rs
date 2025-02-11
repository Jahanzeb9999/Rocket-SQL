use rocket::{get, post, State};
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use serde_json::{json, Value};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Unauthorized access")]
    Unauthorized,
}

// Custom responder for our API errors
pub type ApiResult<T> = Result<Json<T>, status::Custom<Json<Value>>>;

impl From<ApiError> for status::Custom<Json<Value>> {
    fn from(error: ApiError) -> Self {
        let (status, message) = match &error {
            ApiError::Database(_) => (Status::InternalServerError, "Internal server error"),
            ApiError::NotFound(msg) => (Status::NotFound, msg.as_str()),
            ApiError::BadRequest(msg) => (Status::BadRequest, msg.as_str()),
            ApiError::Unauthorized => (Status::Unauthorized, "Unauthorized access"),
        };
        
        status::Custom(
            status,
            Json(json!({
                "error": message
            }))
        )
    }
}

