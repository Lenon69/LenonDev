// src/handlers/admin.rs
use crate::{AppState, components};
use axum::{
    Form,
    Router,
    extract::{Request, State},
    middleware::Next,
    response::{Html, IntoResponse, Redirect, Response},
    routing::get, // 'post' jest teraz poprawnie używany
};
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
    let admin_password = std::env::var("ADMIN_PASSWORD").expect("ADMIN_PASSWORD must be set");
    if form.password == admin_password {
        session.insert("user_id", "admin").await.unwrap();
        Redirect::to("/admin/dashboard")
    } else {
        Redirect::to("/admin/login")
    }
}

async fn logout(session: Session) -> impl IntoResponse {
    session.clear().await;
    Redirect::to("/")
}

async fn dashboard() -> Html<maud::Markup> {
    Html(components::admin::admin_layout(
        "Dashboard",
        maud::html! {
            div class="w-full max-w-4xl bg-slate-900 p-8 rounded-lg shadow-lg text-center" {
                h1 class="text-2xl font-bold mb-6" { "Witaj w panelu admina!" }
                div class="flex flex-col sm:flex-row justify-center items-center gap-6" {
                    a href="/admin/articles/new" class="inline-block bg-purple-600 hover:bg-purple-700 text-white font-bold py-3 px-6 rounded-lg transition-colors text-lg" {
                        "✍️ Utwórz nowy artykuł"
                    }
                    a href="/admin/logout" class="inline-block bg-slate-700 hover:bg-slate-600 text-white font-bold py-2 px-4 rounded-lg transition-colors" {
                        "Wyloguj"
                    }
                }
            }
        },
    ))
}

// Handler GET /admin/articles/new
async fn get_new_article_form() -> Html<maud::Markup> {
    Html(components::admin::new_article_form())
}

// === POPRAWIONY HANDLER POST ===
async fn post_new_article(
    State(state): State<AppState>,
    Form(form): Form<ArticleForm>,
) -> impl IntoResponse {
    // <-- ZMIANA: Używamy prostszego `impl IntoResponse`
    let slug = slug::slugify(&form.title);

    let query_result = sqlx::query(
        "INSERT INTO articles (title, slug, excerpt, content, published_at) VALUES ($1, $2, $3, $4, NOW())"
    )
    .bind(form.title)
    .bind(slug)
    .bind(form.excerpt)
    .bind(form.content)
    .execute(&state.db_pool)
    .await;

    match query_result {
        // Obie gałęzie zwracają typy, które implementują IntoResponse
        Ok(_) => Ok(Redirect::to("/blog")),
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
}
