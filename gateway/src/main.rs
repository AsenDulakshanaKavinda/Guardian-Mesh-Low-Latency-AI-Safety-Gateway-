
mod utils;


fn main() {
    let _guard = utils::logging::init_logging();
    let settings = utils::config::AppConfig::load().expect("Failed to load configuration");

    tracing::info!("new log");
    tracing::info!("the port: {}", settings.port);
    
}

                


