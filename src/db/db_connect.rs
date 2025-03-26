use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use crate::db::config;

pub async fn create_db_pool() -> PgPool {
    let database_url = config::get_database_url();
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to the database")
}
