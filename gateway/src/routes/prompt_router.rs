//! This module defines the routes for handling prompt-related operations in the application.
//! It uses the Axum framework to create a router that maps HTTP methods and paths to
//! handler functions defined in the `prompt_hadlers` module.
//! The `prompt_routes` function creates a new router and defines the following route:
//! - `POST /prompt/create`: Maps to the `insert_prompt` handler for creating a new prompt.
//! Each route is associated with a specific HTTP method (POST) and a path that may include parameters for dynamic routing.
//! The handlers will process the incoming requests and return appropriate responses based on the defined logic in the `prompt_hadlers` module.

use axum::{Router, routing::post};

use crate::handlers::prompt_hadlers::prompt_hadler::insert_prompt;

pub fn prompt_routes() -> Router {
    Router::new().route("/prompt/create", post(insert_prompt))
}
