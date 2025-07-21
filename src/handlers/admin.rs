// src/handlers/admin.rs
use crate::{
    AppState,
    components::{self},
    models::Article,
};
use axum::{
    Form, Router,
    extract::{Path, Request, State},
    middleware::Next,
    response::{Html, IntoResponse, Redirect, Response},
    routing::{get, post},
};
use maud::Markup;
use serde::Deserialize;
use slug; // Upewnij się, że ten import jest
use tower_sessions::Session;

#[derive(Deserialize)]
pub struct LoginForm {
    password: String,
}

#[derive(Deserialize)]
struct ArticleForm {
    title: String,
    excerpt: String,
    content: String,
}

// === HANDLERY ===

async fn get_login_form() -> Html<maud::Markup> {
    Html(components::admin::login_form())
}

async fn post_login(session: Session, Form(form): Form<LoginForm>) -> impl IntoResponse {
    // Odczytujemy hasło ze zmiennej środowiskowej (z pliku .env)
    let admin_password =
        std::env::var("ADMIN_PASSWORD").expect("ADMIN_PASSWORD musi być ustawione w pliku .env");

    // Porównujemy hasło z formularza z tym z .env
    if form.password == admin_password {
        // Hasło poprawne - logujemy, wstawiając do sesji prosty identyfikator
        session.insert("user_id", "admin").await.unwrap();
        Redirect::to("/admin/dashboard")
    } else {
        // Hasło niepoprawne
        Redirect::to("/admin/login")
    }
}

async fn logout(session: Session) -> impl IntoResponse {
    session.clear().await;
    Redirect::to("/")
}

async fn dashboard(State(state): State<AppState>) -> Html<maud::Markup> {
    // Pobieramy wszystkie artykuły z bazy danych, sortując od najnowszych
    let articles = sqlx::query_as::<_, Article>("SELECT * FROM articles ORDER BY created_at DESC")
        .fetch_all(&state.db_pool)
        .await
        .unwrap_or_else(|e| {
            // W przypadku błędu, logujemy go i zwracamy pustą listę
            eprintln!("Błąd podczas pobierania artykułów: {}", e);
            vec![]
        });

    // Przekazujemy pobrane artykuły do nowego komponentu widoku, który zaraz stworzymy
    Html(components::admin::dashboard_view(articles))
}

// Handler GET /admin/articles/new
async fn get_new_article_form() -> Html<maud::Markup> {
    Html(components::admin::new_article_form())
}

// === POPRAWIONY HANDLER POST ===
// Handler dodający nowy artykuł
async fn post_new_article(
    State(state): State<AppState>,
    Form(form): Form<ArticleForm>,
) -> impl IntoResponse {
    let slug = slug::slugify(&form.title);

    let query_result = sqlx::query(
        "INSERT INTO articles (title, slug, excerpt, content, published_at) VALUES ($1, $2, $3, $4, NOW())",
    )
    .bind(form.title)
    .bind(slug)
    .bind(form.excerpt)
    .bind(form.content)
    .execute(&state.db_pool)
    .await;

    match query_result {
        Ok(_) => {
            // KROK 1: Unieważniamy cache strony głównej bloga
            println!("CACHE INVALIDATION: Strona /blog została unieważniona.");
            state.cache.invalidate("page:/blog");
            Ok(Redirect::to("/admin/dashboard"))
        }
        Err(e) => {
            eprintln!("Błąd zapisu do bazy danych: {}", e);
            Err((
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Błąd serwera podczas zapisu artykułu.",
            ))
        }
    }
}

// === MIDDLEWARE ("STRAŻNIK") ===
pub async fn auth_middleware(
    session: Session,
    State(_state): State<AppState>,
    request: Request,
    next: Next,
) -> Response {
    let is_logged_in = session
        .get::<String>("user_id")
        .await
        .map(|user| user.is_some())
        .unwrap_or(false);
    if is_logged_in {
        next.run(request).await
    } else {
        Redirect::to("/admin/login").into_response()
    }
}

// === FUNKCJE TWORZĄCE ROUTERY ===
pub fn public_admin_routes() -> Router<AppState> {
    Router::new().route("/login", get(get_login_form).post(post_login))
}

pub fn protected_admin_routes() -> Router<AppState> {
    Router::new()
        .route("/dashboard", get(dashboard))
        .route("/logout", get(logout))
        .route(
            "/articles/new",
            get(get_new_article_form).post(post_new_article),
        )
        .route(
            "/articles/edit/{id}",
            get(get_edit_article_form).post(post_update_article),
        )
        .route("/articles/delete/{id}", post(post_delete_article))
}

// Handler GET /admin/articles/edit/{id} - wyświetla formularz edycji
async fn get_edit_article_form(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Markup, Redirect> {
    match sqlx::query_as::<_, Article>("SELECT * FROM articles WHERE id = $1")
        .bind(id)
        .fetch_one(&state.db_pool)
        .await
    {
        Ok(article) => Ok(components::admin::edit_article_form(&article)),
        Err(_) => Err(Redirect::to("/admin/dashboard")), // Jeśli nie ma artykułu, przekieruj
    }
}

// Handler POST /admin/articles/edit/{id} - zapisuje zmiany
async fn post_update_article(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Form(form): Form<ArticleForm>,
) -> impl IntoResponse {
    let slug = slug::slugify(&form.title);

    let query_result = sqlx::query(
        "UPDATE articles SET title = $1, slug = $2, excerpt = $3, content = $4, updated_at = NOW() WHERE id = $5",
    )
    .bind(form.title)
    .bind(&slug) // Przekazujemy referencję do sluga
    .bind(form.excerpt)
    .bind(form.content)
    .bind(id)
    .execute(&state.db_pool)
    .await;

    if query_result.is_ok() {
        // KROK 2: Unieważniamy cache strony głównej bloga ORAZ edytowanego artykułu
        println!(
            "CACHE INVALIDATION: Strona /blog i artykuł /blog/{} zostały unieważnione.",
            slug
        );
        state.cache.invalidate("page:/blog");
        state.cache.invalidate(&format!("page:/blog/{}", slug));
    }

    Redirect::to("/admin/dashboard")
}

// Handler POST /admin/articles/delete/{id} - usuwa artykuł
async fn post_delete_article(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    // KROK 3: Najpierw pobieramy slug artykułu, ZANIM go usuniemy
    let article_slug: Option<String> =
        sqlx::query_scalar("SELECT slug FROM articles WHERE id = $1")
            .bind(id)
            .fetch_one(&state.db_pool)
            .await
            .ok();

    // Usuwamy artykuł z bazy danych
    sqlx::query("DELETE FROM articles WHERE id = $1")
        .bind(id)
        .execute(&state.db_pool)
        .await
        .ok();

    // Unieważniamy cache strony głównej bloga
    println!("CACHE INVALIDATION: Strona /blog została unieważniona.");
    state.cache.invalidate("page:/blog");

    // Jeśli udało się pobrać slug, unieważniamy także cache dla tego konkretnego artykułu
    if let Some(slug) = article_slug {
        println!(
            "CACHE INVALIDATION: Artykuł /blog/{} został unieważniony.",
            slug
        );
        state.cache.invalidate(&format!("page:/blog/{}", slug));
    }

    Redirect::to("/admin/dashboard")
}
