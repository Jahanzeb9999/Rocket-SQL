use rocket::get;
use rocket::{http::Status, serde::json::Json, State};


#[get("/health")]
pub fn health_check() -> &'static str {
    "OK"
}

