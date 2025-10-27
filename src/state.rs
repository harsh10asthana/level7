use sqlx::SqlitePool;
use dotenvy::dotenv;
use std::env;

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
}

impl AppState {
    pub async fn new() -> Self {
        dotenv().ok();
        let database_url =  env::var("DATABASE_URL").expect("DATABASE_URL not set");
        let pool = SqlitePool::connect(&format!("{}?mode=rwc", database_url))
            .await
            .expect("Failed to connect to database");
        
        // Create table if it doesn't exist
        sqlx::query("CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY, name TEXT NOT NULL)")
            .execute(&pool)
            .await
            .expect("Failed to create table");

        Self { db: pool }
    }
}