use axum::{Router, routing::get};

use crate::handlers::test_handler;


pub fn user_routes() -> Router {

    Router::new()
        .route("/user", get(test_handler))

}
