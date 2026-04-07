
mod utils;
use crate::utils::config::APP_CONFIG;
fn main() {
    let _guard = utils::logging::init_logging();
    let settings = APP_CONFIG.as_ref().expect("Failed to load configuration");

    // tracing::info!("new log");
    tracing::info!("the port: {}", settings.port);
    
}

                


