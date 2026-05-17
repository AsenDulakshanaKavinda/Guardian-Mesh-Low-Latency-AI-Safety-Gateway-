mod entities;
mod handlers;
mod models;
mod routes;
mod server;
mod utils;

#[tokio::main]
async fn main() {
    server::server().await;
}
