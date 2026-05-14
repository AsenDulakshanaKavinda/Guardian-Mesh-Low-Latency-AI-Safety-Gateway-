
mod server;
mod routes;
mod handlers;
mod utils;
mod models;



#[tokio::main]
async fn main() {

    server::server().await;
}