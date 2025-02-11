use sqlx::MySqlPool;

pub struct AppState {
    pub db: MySqlPool,
}

impl AppState {
    pub fn new(db: MySqlPool) -> Self {
        Self { db }
    }
}