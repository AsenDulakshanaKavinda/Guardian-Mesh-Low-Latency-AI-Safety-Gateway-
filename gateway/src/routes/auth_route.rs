use axum::{Router, routing::get};

use crate::handlers::test_handler;


pub fn auth_routes() -> Router {

    Router::new()
        .route("/auth", get(test_handler))

}
