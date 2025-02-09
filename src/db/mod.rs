use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use crate::config::Config;


pub async fn create_pool() -> Result<MySqlPool, sqlx::Error> {
    let config = Config::from_env();

    MySqlPoolOptions::new()
    .max_connections(5)
    .connect(&config.database_url)
    .await
}


