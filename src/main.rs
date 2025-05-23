use std::{convert::Infallible, env, fs, net::SocketAddr};

use axum::{response::{Html, IntoResponse}, routing::get, Router};
use handlers::generate;
use state::StateStruct;
use tower::service_fn;
use tower_http::services::ServeDir;

mod state;
mod handlers;
mod dto;

#[tokio::main]
async fn main() {
    println!("STARTING EMG...");

    // Get environment variables
    let frontend_path = env::var("EMG_FRONTEND_PATH").unwrap_or(String::from("/opt/emg_frontend"));

    let state = StateStruct::new_state();

    let service = ServeDir::new(&frontend_path).not_found_service(service_fn(move |_req| {
        let dir = frontend_path.clone();
        async move {
            let content = fs::read_to_string(format!("{}/index.html", dir)).unwrap_or_else(|_| {
                "<html><body><h1>Internal Server Error</h1><p>Couldn't read the frontend's index.html</p></body></html>".to_string()
            });
            Ok::<_, Infallible>(Html(content).into_response())
        }
    }
    ));

    let router = Router::new()
        .route("/api/{dif}", get(generate))
        .fallback_service(service)
        .with_state(state)
        // .layer(CorsLayer::permissive())
    ;
    
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("Started!");
    axum::serve(listener, router).await.unwrap();
}
