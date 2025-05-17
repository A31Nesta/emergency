use std::net::SocketAddr;

use axum::{routing::get, Router};
use handlers::generate;
use state::StateStruct;

mod state;
mod handlers;
mod dto;

#[tokio::main]
async fn main() {
    println!("STARTING EMG...");

    let state = StateStruct::new_state();

    let router = Router::new()
        .route("/api/{dif}", get(generate))
        .with_state(state)
    ;
    
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
