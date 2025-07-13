// src/main.rs
use axum::Router;
use axum::routing::{delete, get, patch, post};
use handlers::htmx::get_main_content;
use std::net::SocketAddr;
use tower_http::services::ServeDir;

mod components;
mod handlers;

#[tokio::main]
async fn main() {
    // Serwer, który serwuje wszystko z folderu `static`
    // Axum jest na tyle mądry, że automatycznie poszuka pliku `index.html`
    let app = Router::new()
        .route("/content", get(get_main_content))
        .fallback_service(ServeDir::new("static"));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Serwer nasłuchuje na http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
