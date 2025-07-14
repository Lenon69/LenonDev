// src/main.rs
mod appstate;
mod components;
mod handlers;
mod models;

use axum::middleware;
use axum::routing::post;
use axum::{Router, routing::get};
use axum_server::tls_rustls::RustlsConfig;
use handlers::{
    blog::{blog_index, show_article},
    contact::handle_contact_form,
    projects::get_project_detail,
};
use resend_rs::Resend;
use sqlx::PgPool;
use std::net::SocketAddr;
use tower_http::services::ServeDir; // <-- DODAJ TEN IMPORT

use crate::appstate::AppState;
use crate::handlers::admin::{auth_middleware, protected_admin_routes, public_admin_routes};
use crate::handlers::htmx::get_main_content;

use tower_cookies::CookieManagerLayer; // Import do obsługi ciasteczek
use tower_sessions::{MemoryStore, SessionManagerLayer}; // Import do obsługi sesji

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

    // Inicjalizacja klienta Resend
    let resend_api_key = std::env::var("RESEND_API_KEY").expect("RESEND_API_KEY must be set");
    let resend_client = Resend::new(&resend_api_key);

    // --- NOWA KONFIGURACJA SESJI ---
    // Tworzymy magazyn sesji w pamięci.
    let session_store = MemoryStore::default();
    // Tworzymy warstwę sesji.
    // Klucz sesji jest generowany losowo, co jest w porządku dla deweloperki.
    // W produkcji warto ustawić go na stałe z pliku .env.
    let session_layer = SessionManagerLayer::new(session_store).with_secure(false);
    // ---------------------------------

    // Tworzenie stanu aplikacji
    let app_state = AppState {
        db_pool,
        resend_client,
    };

    let app = Router::new()
        .route("/content", get(get_main_content))
        .route("/contact", post(handle_contact_form))
        .route("/project/{id}", get(get_project_detail))
        .route("/blog", get(blog_index))
        .route("/blog/{slug}", get(show_article))
        .nest(
            "/admin",
            // Najpierw łączymy trasy chronione z ich warstwą middleware
            protected_admin_routes()
                .route_layer(middleware::from_fn_with_state(
                    app_state.clone(),
                    auth_middleware,
                ))
                // Następnie dołączamy trasy publiczne, które nie mają tej warstwy
                .merge(public_admin_routes()),
        )
        .fallback_service(ServeDir::new("static"))
        .with_state(app_state)
        .layer(session_layer)
        .layer(CookieManagerLayer::new());

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
