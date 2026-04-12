use serde::Deserialize;

// todo - add more config fields as needed (e.g., logging, caching, etc.)
#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub logging: LoggingConfig, 
    pub host_config: HostConfig,
}

#[derive(Debug, Deserialize)]
pub struct LoggingConfig {
    pub log_file: String,
    pub log_dir: String,
}

#[derive(Debug, Deserialize)]
pub struct HostConfig {
    pub host: String,
    pub addr: String,
}