use std::{env, net::SocketAddr};

use axum::Router;
use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
async fn main() {
    println!("STARTING EMG...");

    // Get environment variables
    let frontend_path = env::var("EMG_FRONTEND_PATH").unwrap_or(String::from("/opt/emg_frontend"));

    let serve_dir = ServeDir::new(&frontend_path);
    let fallback = ServeFile::new(format!("{}/index.html", frontend_path));

    let router = Router::new()
        .fallback_service(serve_dir.fallback(fallback))
        // .layer(CorsLayer::permissive())
    ;
    
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("Started!");
    axum::serve(listener, router).await.unwrap();
}