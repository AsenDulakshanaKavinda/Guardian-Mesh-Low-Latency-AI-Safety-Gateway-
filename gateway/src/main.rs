
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

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind tcp listener.");

    tracing::info!("Server running on http://localhost:3000");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");

}

                


