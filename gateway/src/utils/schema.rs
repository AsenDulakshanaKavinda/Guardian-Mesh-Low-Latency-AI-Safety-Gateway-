use serde::Deserialize;

// todo - add more config fields as needed (e.g., logging, caching, etc.)
#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub logging: LoggingConfig, // Nested!
}

#[derive(Debug, Deserialize)]
pub struct LoggingConfig {
    pub log_file: String,
    pub log_dir: String,
}