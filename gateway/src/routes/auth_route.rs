//! This module defines the routes for handling authentication-related operations in the application.
//! It uses the Axum framework to create a router that maps HTTP methods and paths to
//! handler functions defined in the `auth_handlers` module.
//! The `auth_routes` function creates a new router and defines the following routes:
//! - `POST /user/login`: Maps to the `login_user` handler for user login.
//! - `POST /user/register`: Maps to the `register_user` handler for user registration.
//! Each route is associated with a specific HTTP method (POST) and a path that may include parameters for dynamic routing.
//! The handlers will process the incoming requests and return appropriate responses based on the defined logic in the `auth_handlers` module.  

use axum::{Router, routing::post};

use crate::handlers::auth_handlers::login_handler::login_user;
use crate::handlers::auth_handlers::register_handler::register_user;

pub fn auth_routes() -> Router {
    Router::new()
        .route("/user/login", post(login_user))
        .route("/user/register", post(register_user))
}
