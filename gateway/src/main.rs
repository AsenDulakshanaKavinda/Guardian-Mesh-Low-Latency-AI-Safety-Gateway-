
mod server;
mod routes;
mod handlers;
mod utils;
mod models;
mod entities;



#[tokio::main]
async fn main() {

    server::server().await;
}