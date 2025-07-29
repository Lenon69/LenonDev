// src/main.rs
mod appstate;
mod components;
mod handlers;
mod models;

use appstate::CacheValue;
use axum::extract::Query;
use axum::http::{HeaderMap, HeaderValue, Uri, header};
use axum::middleware;
use axum::response::Html;
use axum::routing::post;
use axum::{Router, routing::get};
use components::{layout, sections};
use handlers::blog_cms::get_blog_cms_page;
use handlers::custom_project::get_custom_project_page;
use handlers::ecommerce::get_ecommerce_page;
use handlers::error::handler_404;
use handlers::hosting::get_hosting_page;
use handlers::htmx::ScrollParams;
use handlers::landing_page::get_landing_page;
use handlers::maintenance::get_maintenance_page;
use handlers::offer::get_offer_page;
use handlers::privacy::get_privacy_policy_page;
use handlers::projects::show_project;
use handlers::seo::get_sitemap;
use handlers::seo_optimization::get_seo_optimization_page;
use handlers::simple_site::get_simple_site_page;
use handlers::web_app::get_web_app_page;
use handlers::{
    blog::{blog_index, show_article},
    contact::handle_contact_form,
};
use maud::PreEscaped;
use models::{Article, ArticleSchema, Author, ImageObject, Project, ProjectWithImages, Publisher};
use moka::sync::Cache;
use resend_rs::Resend;
use sqlx::PgPool;
use std::net::SocketAddr;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::set_header::SetResponseHeaderLayer;

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
        // --- NOWE TRASY DLA USŁUG ---
        .route("/oferta/opieka", get(get_maintenance_page))
        .route("/oferta/seo", get(get_seo_optimization_page))
        .route("/oferta/hosting", get(get_hosting_page))
        .route("/oferta/prosta-strona-wizytowka", get(get_simple_site_page))
        .route("/oferta/landing-page", get(get_landing_page))
        .route("/oferta/blog-cms", get(get_blog_cms_page))
        .route("/oferta/sklep-internetowy", get(get_ecommerce_page))
        .route("/oferta/projekt-indywidualny", get(get_custom_project_page))
        .route("/oferta/aplikacja-webowa-crm", get(get_web_app_page))
        // --- KONIEC NOWYCH TRAS ---
        .route("/polityka-prywatnosci", get(get_privacy_policy_page))
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
        .layer(SetResponseHeaderLayer::if_not_present(
            header::CACHE_CONTROL,
            HeaderValue::from_static("public, max-age=32140800"),
        ))
        .route_service("/", ServeFile::new("static/index.html"))
        .route_service("/robots.txt", ServeFile::new("static/robots.txt"))
        // Następnie łączymy (merge) wszystkie trasy naszej aplikacji
        .merge(app_router)
        .with_state(app_state)
        .layer(session_layer)
        .layer(CookieManagerLayer::new());

    // --- KONIEC NOWEJ STRUKTURY ROUTERA ---

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("🚀 Serwer nasłuchuje na http://{}", addr);

    // Tworzymy listener TCP dla serwera HTTP
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    // Uruchamiamy serwer Axum, który będzie obsługiwał przychodzące połączenia
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
    // --- KONIEC STARTU SERWERA ---}
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
    let offer_params = Query(ScrollParams { scroll_to: None });
    let offer_uri: Uri = "/oferta".parse().unwrap();
    let offer_page = get_offer_page(
        offer_uri,
        HeaderMap::new(),
        axum::extract::State(state.clone()),
        offer_params,
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

    // 6. Cache'owanie wpisów z bloga

    println!(" caching articles...");
    const ARTICLES_PER_PAGE: i64 = 5;
    if let Ok(articles) = sqlx::query_as::<_, Article>(
        "SELECT * FROM articles WHERE published_at IS NOT NULL ORDER BY published_at DESC LIMIT $1",
    )
    .bind(ARTICLES_PER_PAGE)
    .fetch_all(&state.db_pool)
    .await
    {
        for article in articles {
            let sections: Vec<String> = article
                .content
                .split("---")
                .filter(|s| !s.trim().is_empty())
                .map(|raw_html_section| raw_html_section.trim().to_string())
                .collect();

            let content_fragment = maud::html! {
                div class="container mx-auto px-4 pb-16 lg:pb-24 pt-36 md:pt-28" {
                    div class="max-w-4xl mx-auto" {
                        div class="text-center mb-12" {
                            @if let Some(published_at) = article.published_at {
                                p class="text-sm text-slate-400 mb-4" { (published_at.format("%d %B %Y")) }
                            }
                            h1 class="text-4xl lg:text-6xl font-bold tracking-tighter bg-gradient-to-r from-brand-cyan to-brand-green text-transparent bg-clip-text py-4" {
                                (article.title)
                            }
                        }
                        @for rendered_html in &sections {
                            section class="py-10" {
                                div class="prose prose-invert prose-xl" { (PreEscaped(rendered_html)) }
                            }
                        }
                        div class="text-center mt-16" {
                            a href="/blog" hx-get="/blog" hx-target="#content-area" hx-push-url="/blog" class="cursor-pointer inline-block bg-slate-700 hover:bg-slate-600 transition-colors text-white font-bold py-2 px-6 rounded-lg" { "← Wróć na bloga" }
                        }
                    }
                }
            };

            let base_url = std::env::var("APP_BASE_URL").unwrap_or_default();
            let og_image_url = format!("{}/public/og-image.avif", base_url);
            let schema = ArticleSchema {
                context: "https://schema.org",
                type_of: "BlogPosting",
                headline: article.title.clone(),
                image: ImageObject {
                    type_of: "ImageObject",
                    url: og_image_url,
                    width: 1200,
                    height: 630,
                },
                author: Author {
                    type_of: "Person".to_string(),
                    name: "Lenon".to_string(),
                },
                date_published: article
                    .published_at
                    .map(|d| d.to_rfc3339())
                    .unwrap_or_default(),
                publisher: Publisher {
                    type_of: "Organization",
                    name: "LenonDev",
                    logo: ImageObject {
                        type_of: "ImageObject",
                        url: format!("{}/public/fixed-logo.avif", base_url),
                        width: 372,
                        height: 281,
                    },
                },
                date_modified: article.updated_at.to_rfc3339(),
            };
            let schema_json = serde_json::to_string(&schema).ok();

            let full_page_html = Html(layout::base_layout(
                &article.title,
                content_fragment,
                article.excerpt.as_deref(),
                schema_json,
                &format!("/blog/{}", article.slug),
            ));

            let cache_key = format!("page:/blog/{}", article.slug);
            state
                .cache
                .insert(cache_key, (HeaderMap::new(), full_page_html.clone()));
        }
    }

    println!("✅ Cache rozgrzany!");
}
