// Imports
use axum::serve;
use std::net::SocketAddr;

// mods
mod utils;
mod router;
mod models;

// Crates
use crate::{
    router::router::create_router,
    utils::db::connect_db
};

#[tokio::main]
async fn main() {
    let port: u16 = 8000;

    let _client = connect_db().await;

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    println!("The server is up on route: {addr}");
    println!("The database is connected successfully");

    let app = create_router().await;

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    serve(listener, app).await.unwrap();
}
