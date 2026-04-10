use axum::{Router, routing::get};

use crate::handlers::home::home;
use crate::handlers::health::health_check;

// This module defines the routes for the chat application and creates the Axum router.
// The `create_app` function sets up the routes for the application, 
// including the home page and health check endpoint.
pub fn create_app() -> Router {
    Router::new()
        .route("/", get(home))
        .route("/health", get(health_check))

}
