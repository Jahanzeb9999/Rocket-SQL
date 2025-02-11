use rocket::State;
use crate::error::ApiError;
use crate::models::user::{User, CreateUser};
use crate::state::AppState;

pub async fn get_user_by_id(
    state: &State<AppState>,
    user_id: i32,
) -> Result<User, ApiError> {
    let user = sqlx::query_as::<_, User>(
        "SELECT id, name, email FROM users WHERE id = ?"
    )
    .bind(user_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| ApiError::NotFound(format!("User {} not found", user_id)))?;
    
    Ok(user)
}

pub async fn insert_user(
    state: &State<AppState>,
    user: CreateUser,
) -> Result<User, ApiError> {
    let result = sqlx::query!(
        "INSERT INTO users (name, email) VALUES (?, ?)",
        user.name,
        user.email
    )
    .execute(&state.db)
    .await?;
    
    let id = result.last_insert_id() as i32;
    
    Ok(User {
        id,
        name: user.name,
        email: user.email,
    })
}
