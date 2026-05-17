//! This module defines the main router for the application, combining all individual route modules.
//! It imports the necessary route modules (e.g., `auth_route`, `user_route`, `prompt_router`) and
//!     defines a function `main_route` that creates a new router.
//! The `main_route` function establishes a connection to the database, sets up CORS policies,
//!     and merges the individual route modules into a single router that can be used by the application to handle incoming HTTP requests.
//! The main router also applies middleware for request guarding and includes the database connection as an extension for use in the route handlers.              

use axum::http::Method;
use axum::{Extension, Router, middleware};
use sea_orm::{Database, DatabaseConnection};
use tower_http::cors::{Any, CorsLayer};

mod auth_route;
mod prompt_router;
mod user_route;
use crate::routes::auth_route::auth_routes;
use crate::routes::prompt_router::prompt_routes;
// use crate::routes::user_route::user_routes;
use crate::utils;
use crate::utils::guards::guard;

pub async fn main_route() -> Router {
    let conn_str = (*utils::constants::DATABASE_URL).clone();

    let db: DatabaseConnection = Database::connect(conn_str)
        .await
        .expect("Error while connecting to database.");

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(Any);

    let main_route = Router::new()
        // .merge(user_routes())
        .merge(prompt_routes())
        .route_layer(middleware::from_fn(guard))
        .merge(auth_routes())
        .layer(Extension(db))
        .layer(cors);

    main_route
}
