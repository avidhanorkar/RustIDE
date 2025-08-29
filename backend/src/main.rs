// Imports
use axum::serve;
use std::net::SocketAddr;

// mods
mod router;


// Crates
use crate::router::{router::create_router};

#[tokio::main]
async fn main() {

    let port: u16 = 8000;

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    println!("The server is up on route: {addr}");

    let app = create_router().await;

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    serve(listener, app).await.unwrap();
}
