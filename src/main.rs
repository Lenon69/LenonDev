// src/main.rs
mod components;
mod handlers;

use axum::{Router, routing::get};
use axum_server::tls_rustls::RustlsConfig;
use handlers::htmx::get_main_content;
use std::net::SocketAddr;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/content", get(get_main_content))
        .fallback_service(ServeDir::new("static"));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Serwer nasłuchuje na https://{}", addr);

    // Konfiguracja TLS
    let config = RustlsConfig::from_pem_file("localhost.pem", "localhost-key.pem")
        .await
        .unwrap();

    // Uruchomienie serwera z obsługą HTTP/1.1, HTTP/2 i HTTP/3
    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
