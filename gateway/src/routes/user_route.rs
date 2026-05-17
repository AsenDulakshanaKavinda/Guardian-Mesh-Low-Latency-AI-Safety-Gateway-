//! This module defines the routes for user-related operations in the application.
//! It uses the Axum framework to create a router that maps HTTP methods and paths to
//! handler functions defined in the `user_handlers` module.
//!
//! The `user_routes` function creates a new router and defines the following routes:
//! - `POST /user/create`: Maps to the `create_user` handler for creating a new user.
//! - `POST /user/fetch`: Maps to the `fetch_user` handler for fetching user details.
//! - `PUT /user/update/{uuid}`: Maps to the `update_user` handler for updating user information based on a unique identifier (UUID).
//! - `DELETE /user/delete/{uuid}`: Maps to the `delete_user` handler for deleting a user based on a unique identifier (UUID).
//! Each route is associated with a specific HTTP method (POST, PUT, DELETE) and a path that may include parameters (e.g., `{uuid}`) for dynamic routing.
//! The handlers will process the incoming requests and return appropriate responses based on the defined logic in the `user_handlers` module.    

use axum::{
    Router,
    routing::{delete, post, put},
};

use crate::handlers::user_handlers::{create_user, delete_user, fetch_user, update_user};

pub fn user_routes() -> Router {
    Router::new()
        .route("/user/create", post(create_user))
        .route("/user/fetch", post(fetch_user))
        .route("/user/update/{uuid}", put(update_user))
        .route("/user/delete/{uuid}", delete(delete_user))
}
