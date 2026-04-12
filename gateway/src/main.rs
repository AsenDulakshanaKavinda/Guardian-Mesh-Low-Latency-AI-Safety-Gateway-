mod utils; 
mod handlers;
mod routes;
mod services;

use crate::routes::chat::create_app;
use crate::utils::config::APP_CONFIG;
use crate::utils::logging::init_logging;

#[tokio::main]
async fn main() {
    let _guard = init_logging();
    let settings = APP_CONFIG.as_ref().expect("Failed to load configuration");

    let app = create_app();
    let addr = &settings.host_config.addr;
    let host = &settings.host_config.host;


    let listener = tokio::net::TcpListener::bind(addr.to_string())
        .await
        .expect("Failed to bind tcp listener.");

    tracing::info!("Server running on {}", host.to_string());

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");

}

                


