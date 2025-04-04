use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use crate::db::config;

pub async fn create_db_pool() -> PgPool {
    let load_env_output = config::load_env();
    let mut database_url = String::new();
    
    match load_env_output {
        Ok(url) => database_url = url,
        Err(_) => println!("Failed to load environment variables or DATABASE_URL env var is missing."),
    }

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to the database")
}
