use dotenvy::dotenv;
use std::env;

pub fn load_env() {
    dotenv().ok();
    println!("Loaded environment variables.");
}

pub fn get_database_url() -> String {
    env::var("DATABASE_URL").expect("Expected DATABASE_URL to be set. (Not found)")
}
