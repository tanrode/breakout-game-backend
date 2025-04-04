use dotenvy::dotenv;
use std::env;

pub fn load_env() -> Result<String, std::env::VarError> {
    dotenv().ok();

    match env::var("DATABASE_URL") {
        Ok(value) => {
            println!("Loaded environment variables..");
            Ok(value)
        }
        Err(_) => {
            println!("Failed to load environment variables or DATABASE_URL env var is missing.");
            Err(std::env::VarError::NotPresent)
        }
    }
}
