// src/handlers/admin.rs
use crate::{AppState, components, models::Article, models::User};
use axum::{
    Form,
    Router,
    extract::{Request, State},
    middleware::Next,
    response::{Html, IntoResponse, Redirect, Response},
    routing::get, // 'post' jest teraz poprawnie używany
};
use bcrypt::verify;
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

// Zastąp całą funkcję post_login
async fn post_login(
    State(state): State<AppState>,
    session: Session,
    Form(form): Form<LoginForm>,
) -> impl IntoResponse {
    // Szukamy użytkownika 'admin' w bazie danych
    let user_result = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = 'admin'")
        .fetch_optional(&state.db_pool)
        .await;

    match user_result {
        Ok(Some(user)) => {
            // Weryfikujemy, czy podane hasło pasuje do hasha w bazie
            if verify(&form.password, &user.password_hash).unwrap_or(false) {
                // Hasło poprawne - logujemy
                session.insert("user_id", user.id).await.unwrap();
                Redirect::to("/admin/dashboard")
            } else {
                // Hasło niepoprawne
                Redirect::to("/admin/login")
            }
        }
        _ => {
            // Użytkownik 'admin' nie istnieje lub wystąpił błąd bazy
            eprintln!("Logowanie nieudane: nie znaleziono użytkownika 'admin' lub błąd bazy.");
            Redirect::to("/admin/login")
        }
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
