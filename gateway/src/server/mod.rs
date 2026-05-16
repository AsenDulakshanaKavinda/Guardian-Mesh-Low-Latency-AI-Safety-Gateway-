
use std::net::SocketAddr;

use crate::routes::main_route;


pub async fn server() {

    // address
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // listener
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();

    // main routes
    let main_routes = main_route().await;

    println!(">>> Listening on - {addr} \n");

    // start server
    axum::serve(listener, main_routes)
        .await
        .unwrap();

}
