use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use std::env;

pub async fn connect_db() -> SqlitePool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&database_url) 
            .await
            .expect("Failed to create pool.")
}