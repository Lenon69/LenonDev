// src/main.rs
mod appstate;
mod components;
mod handlers;
mod models;

use appstate::CacheValue;
use axum::extract::Query;
use axum::http::{HeaderMap, Uri};
use axum::middleware;
use axum::response::Html;
use axum::routing::post;
use axum::{Router, routing::get};
use axum_server::tls_rustls::RustlsConfig;
use components::{layout, sections};
use handlers::error::handler_404;
use handlers::offer::get_offer_page;
use handlers::projects::show_project;
use handlers::seo::get_sitemap;
use handlers::{
    blog::{blog_index, show_article},
    contact::handle_contact_form,
};
use models::{Project, ProjectWithImages};
use moka::sync::Cache;
use resend_rs::Resend;
use sqlx::PgPool;
use std::net::SocketAddr;
use tower_http::services::{ServeDir, ServeFile};

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

    // --- NOWA, ODPORNA NA BŁĘDY STRUKTURA ROUTERA ---

    // 1. Definiujemy router tylko dla logiki aplikacji
    let app_router = Router::new()
        .route("/content", get(get_main_content))
        .route("/oferta", get(get_offer_page))
        .route("/blog", get(blog_index))
        .route("/uses", get(get_uses_content))
        .route("/blog/{slug}", get(show_article))
        .route("/projekty/{slug}", get(show_project))
        .route("/contact", post(handle_contact_form))
        .route("/sitemap.xml", get(get_sitemap))
        .nest(
            "/admin",
            protected_admin_routes()
                .route_layer(middleware::from_fn_with_state(
                    app_state.clone(),
                    auth_middleware,
                ))
                .merge(public_admin_routes()),
        )
        .fallback(handler_404); // Fallback dla tras aplikacji

    // 2. Tworzymy główną aplikację, łącząc serwowanie plików z logiką aplikacji
    let app = Router::new()
        // Najpierw obsługujemy pliki statyczne. To ma teraz najwyższy priorytet.
        .nest_service("/public", ServeDir::new("static"))
        .route_service("/", ServeFile::new("static/index.html"))
        .route_service("/robots.txt", ServeFile::new("static/robots.txt"))
        // Następnie łączymy (merge) wszystkie trasy naszej aplikacji
        .merge(app_router)
        .with_state(app_state)
        .layer(session_layer)
        .layer(CookieManagerLayer::new());

    // --- KONIEC NOWEJ STRUKTURY ROUTERA ---

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
    let offer_uri: Uri = "/oferta".parse().unwrap();
    let offer_page = get_offer_page(
        offer_uri,
        HeaderMap::new(),
        axum::extract::State(state.clone()),
    )
    .await;
    state.cache.insert("page:/oferta".to_string(), offer_page);

    // 3. Strona /uses
    let uses_uri: Uri = "/uses".parse().unwrap();
    let uses_page = get_uses_content(
        uses_uri,
        HeaderMap::new(),
        axum::extract::State(state.clone()),
    )
    .await;
    state.cache.insert("page:/uses".to_string(), uses_page);

    // 4. Strona /blog
    let blog_pagination = Query(models::PaginationParams { page: 1 });
    let blog_page = blog_index(
        HeaderMap::new(),
        axum::extract::State(state.clone()),
        blog_pagination,
    )
    .await;
    state.cache.insert("page:/blog".to_string(), blog_page);

    // --- NOWA SEKCJA: Rozgrzewanie cache'a dla projektów ---
    println!(" caching projects...");
    if let Ok(projects) = sqlx::query_as::<_, Project>("SELECT * FROM projects")
        .fetch_all(&state.db_pool)
        .await
    {
        for project in projects {
            let additional_images = sqlx::query_scalar::<_, String>(
                "SELECT image_url FROM project_images WHERE project_id = $1 ORDER BY id",
            )
            .bind(project.id)
            .fetch_all(&state.db_pool)
            .await
            .unwrap_or_else(|_| vec![]);

            let project_with_images = ProjectWithImages {
                id: project.id,
                title: project.title.clone(),
                slug: project.slug.clone(),
                description: project.description.clone(),
                technologies: project.technologies.clone(),
                image_url: project.image_url.clone(),
                project_url: project.project_url.clone(),
                images: additional_images,
            };

            let fragment = sections::project_detail_page(project_with_images);
            let full_page = Html(layout::base_layout(
                &project.title,
                fragment,
                Some(&project.description),
                None,
                &format!("/projekty/{}", project.slug),
            ));

            let cache_key = format!("page:/projekty/{}", project.slug);
            state.cache.insert(cache_key, (HeaderMap::new(), full_page));
        }
    }
    // --- KONIEC SEKCJI DLA PROJEKTÓW ---

    println!("✅ Cache rozgrzany!");
}
