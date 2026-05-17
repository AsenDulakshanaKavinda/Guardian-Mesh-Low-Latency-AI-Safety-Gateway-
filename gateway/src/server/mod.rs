//! This module defines the server setup and configuration for the application.
//! It uses the Axum framework to create an HTTP server that listens for
//! incoming requests and routes them to the appropriate handlers defined in the `routes` module.
//! The `server` function initializes the server by binding to a specified address and port,
//! setting up the main routes using the `main_route` function, and starting the server to listen for incoming requests.
//! The server is configured to handle requests asynchronously using Tokio, allowing for efficient handling of multiple concurrent connections.
//! The server also includes error handling to ensure that any issues during server startup or request processing are properly logged and managed.

use std::net::SocketAddr;

use crate::routes::main_route;

pub async fn server() {
    // address
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // listener
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    // main routes
    let main_routes = main_route().await;

    println!(">>> Listening on - {addr} \n");

    // start server
    axum::serve(listener, main_routes).await.unwrap();
}
