use std::env::{self, VarError};
use dotenvy::dotenv;

// Reads an environment variable from the .env file or the system environment variables.
// # Arguments
// * `key` - The name of the environment variable to read.
// # Returns
// * `Ok(String)` - The value of the environment variable if it exists.
// * `Err(env::VarError)` - An error if the environment variable does not exist or cannot be read.
// # Example
// ```
// let database_url = read_env("DATABASE_URL").expect("DATABASE_URL must be set");
// ```  

pub async fn read_env(key: &str) -> Result<String, VarError> {
    dotenv().ok();
    Ok(env::var(key)?)
}