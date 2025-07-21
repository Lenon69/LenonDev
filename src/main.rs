// src/main.rs
mod appstate;
mod components;
mod handlers;
mod models;

use appstate::CacheValue;
use axum::extract::Query;
use axum::http::HeaderMap;
use axum::middleware;
use axum::routing::post;
use axum::{Router, routing::get};
use axum_server::tls_rustls::RustlsConfig;
use handlers::error::handler_404;
use handlers::offer::get_offer_page;
use handlers::projects::show_project;
use handlers::{
    blog::{blog_index, show_article},
    contact::handle_contact_form,
};
use moka::sync::Cache;
use resend_rs::Resend;
use sqlx::PgPool;
use std::net::SocketAddr;
use tower_http::services::ServeDir;

use crate::appstate::AppState;
use crate::handlers::admin::{auth_middleware, protected_admin_routes, public_admin_routes};
use crate::handlers::htmx::get_main_content;
use crate::handlers::uses::get_uses_content;

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

    // --- Inicjalizacja Cache ---
    let app_cache: Cache<String, CacheValue> = Cache::builder()
        .max_capacity(100)
        .time_to_live(std::time::Duration::from_secs(3600 * 24))
        .build();
    // -------------------------

    // Tworzenie stanu aplikacji
    let app_state = AppState {
        db_pool,
        resend_client,
        cache: app_cache,
    };

    // --- ROZGRZEWANIE CACHE ---
    warm_up_cache(app_state.clone()).await;
    // --------------------------

    let app = Router::new()
        .route("/content", get(get_main_content))
        .route("/contact", post(handle_contact_form))
        .route("/oferta", get(get_offer_page))
        .route("/projekty/{slug}", get(show_project))
        .route("/blog", get(blog_index))
        .route("/uses", get(get_uses_content))
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
        // KROK 1: Serwuj pliki z folderu 'static' pod adresem '/public'
        .nest_service("/public", ServeDir::new("static"))
        // KROK 2: Ustaw nasz handler jako domyślną stronę dla wszystkich innych tras
        .fallback(handler_404)
        .with_state(app_state)
        .layer(session_layer)
        .layer(CookieManagerLayer::new());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
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

// Funkcja rozgrzewająca cache
async fn warm_up_cache(state: AppState) {
    println!("🔥 Rozgrzewanie cache'a...");

    // 1. Strona główna
    let main_page_params = Query(handlers::htmx::ScrollParams { scroll_to: None });
    let main_page = get_main_content(axum::extract::State(state.clone()), main_page_params).await;
    state
        .cache
        .insert("page:/:scroll_to=".to_string(), main_page);

    // 2. Strona /oferta
    let offer_page = get_offer_page(HeaderMap::new(), axum::extract::State(state.clone())).await;
    state.cache.insert("page:/oferta".to_string(), offer_page);

    // 3. Strona /uses
    let uses_page = get_uses_content(HeaderMap::new(), axum::extract::State(state.clone())).await;
    state.cache.insert("page:/uses".to_string(), uses_page);

    // 4. Strona /blog
    let blog_page = blog_index(HeaderMap::new(), axum::extract::State(state.clone())).await;
    state.cache.insert("page:/blog".to_string(), blog_page);

    println!("✅ Cache rozgrzany!");
}
