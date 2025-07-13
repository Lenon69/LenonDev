// src/main.rs

// Deklarujemy nasze nowe moduły
mod components;
mod routes;

use axum::{Router, routing::get};
use std::net::SocketAddr;
use tower_http::services::ServeDir;
// Importujemy nasz handler z nowego modułu
use routes::home::root_handler;

#[tokio::main]
async fn main() {
    let static_service = ServeDir::new("static");

    let app = Router::new()
        .route("/", get(root_handler)) // Używamy zaimportowanego handlera
        .nest_service("/static", static_service);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Serwer nasłuchuje na http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
