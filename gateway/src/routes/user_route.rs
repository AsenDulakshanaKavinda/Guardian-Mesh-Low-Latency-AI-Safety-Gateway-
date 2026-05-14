use axum::{Router, routing::post};

use crate::handlers::user_handlers::{create_user};


pub fn user_routes() -> Router {

    Router::new()
        .route("/user/create", post(create_user))

}
