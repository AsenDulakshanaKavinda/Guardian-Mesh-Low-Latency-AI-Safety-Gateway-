use config::{Config, ConfigError, File};
use once_cell::sync::Lazy;
use crate::utils::schema::AppConfig;

// Global static for app configuration
// This allows us to load the configuration once and access it throughout the app without needing to pass it around.
// example usage: let config = APP_CONFIG.as_ref().expect("Failed to load config");
pub static APP_CONFIG: Lazy<Result<AppConfig, ConfigError>> = Lazy::new(|| {
    AppConfig::load()
});


// Implementation of AppConfig with a method to load configuration from files and environment variables
// The load method reads the RUN_MODE environment variable to determine which config file to load (e.g., dev, prod). 
// It first loads a default config file and then overrides it with an environment-specific config file if it exists
// returns: 
//      A Result containing the AppConfig or a ConfigError if loading fails
// note:
//      The config files should be placed in a "configs" directory at the root of the project, 
//      with names like "defaults.toml", "dev.toml", "prod.toml", etc.
impl AppConfig {
    pub fn load() -> Result<Self, ConfigError> {
        let run_mode = std::env::var("RUN_MODE")
            .unwrap_or_else(|_| "dev".into());

        let config = Config::builder()
            // - load the base/dev config file
            .add_source(File::with_name("configs/defaults"))
            // - load env specific config file
            .add_source(File::with_name(&format!("configs/{}", run_mode)).required(false))
            .build()?;

        config.try_deserialize()
    }
}

