use std::env;

use dotenvy::dotenv;



pub fn read_env(key: &str) -> Result<String, env::VarError> {
    dotenv().ok();

    env::var(key)
}