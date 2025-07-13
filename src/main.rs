// src/main.rs
mod appstate;
mod components;
mod handlers;
mod models;

use axum::routing::post;
use axum::{Router, routing::get};
use axum_server::tls_rustls::RustlsConfig;
use sqlx::PgPool;
use std::net::SocketAddr;
use tower_http::services::ServeDir;

use crate::appstate::AppState;
use crate::handlers::contact::handle_contact_form;
use crate::handlers::htmx::get_main_content;

#[tokio::main]
async fn main() {
    // Ładowanie zmiennych środowiskowych z pliku .env
    dotenvy::dotenv().expect("Failed to load .env file");

    let provider = rustls::crypto::aws_lc_rs::default_provider();
    if let Err(e) = provider.install_default() {
        tracing::error!(
            "Błąd podczas instalacji domyślnego dostawcy kryptograficznego: {:?}",
            e
        );
        std::process::exit(1);
    }

    tracing::info!("Inicjalizacja serwera...");

    // Tworzenie puli połączeń do bazy danych
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = PgPool::connect(&db_url)
        .await
        .expect("Failed to connect to database");

    // Tworzenie stanu aplikacji
    let app_state = AppState { db_pool };

    let app = Router::new()
        .route("/content", get(get_main_content))
        .route("/contact", post(handle_contact_form))
        .fallback_service(ServeDir::new("static"))
        .with_state(app_state);

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
