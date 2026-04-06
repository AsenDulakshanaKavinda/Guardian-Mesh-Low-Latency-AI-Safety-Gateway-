use config::{Config, ConfigError, File, Environment};
use serde::Deserialize;


impl AppConfig {
    pub fn load() -> Result<Self, ConfigError> {
        let run_mode = std::env::var("RUN_MODE")
            .unwrap_or_else(|_| "dev".into());

        let s = Config::builder()
            // - load the base/dev config file
            .add_source(File::with_name("configs/defaults"))
            // - load env specific config file
            .add_source(File::with_name(&format!("configs/{}", run_mode)).required(false))
            .build()?;

        s.try_deserialize()
    }
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub port: u16,
    pub database_url: String,
    pub security: SecurityConfig, // Nested!
}

#[derive(Debug, Deserialize)]
pub struct SecurityConfig {
    pub max_retries: u32,
    pub allow_list: Vec<String>,
}