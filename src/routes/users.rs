use rocket::response::status;
use rocket::{get, post, State};
use rocket::serde::json::Json;
use serde_json::Value;

use crate::error::{ApiError, ApiResult};
use crate::handlers;
use crate::models::user::{User, CreateUser};
use crate::state::AppState;


#[get("/users/<id>")]
pub async fn get_user(state: &State<AppState>, id: i32) -> ApiResult<User> {
    let user = handlers::user::get_user_by_id(state, id)
        .await
        .map_err(|e: ApiError| <ApiError as Into<status::Custom<Json<Value>>>>::into(e))?;
    
    Ok(Json(user))
}

#[post("/users", data = "<user>")]
pub async fn create_user(state: &State<AppState>, user: Json<CreateUser>) -> ApiResult<User> {
    let user = handlers::user::insert_user(state, user.into_inner())
        .await
        .map_err(|e: ApiError| <ApiError as Into<status::Custom<Json<Value>>>>::into(e))?;
    
    Ok(Json(user))
}
