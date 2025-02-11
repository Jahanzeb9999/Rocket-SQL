#[macro_use] extern crate rocket;

mod config;
mod db;
mod error;
mod handlers;
mod models;
mod routes;
mod state;

use routes::health::health_check;
use routes::users::{get_user, create_user};

#[launch]
async fn rocket() -> _ {
    dotenv::dotenv().ok();
    
    let db_pool = db::create_pool().await.expect("Failed to create database pool");
    let app_state = state::AppState::new(db_pool);
    
    rocket::build()
        .manage(app_state)
        .mount("/api", routes![health_check, get_user, create_user])
}
