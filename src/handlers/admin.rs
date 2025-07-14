// src/handlers/admin.rs
use crate::{AppState, components};
use axum::{
    Form, Router,
    extract::{Request, State},
    middleware::Next,
    response::{Html, IntoResponse, Redirect, Response},
    routing::{get, post},
};
use serde::Deserialize;
use tower_sessions::Session;

#[derive(Deserialize)]
pub struct LoginForm {
    password: String,
}

// === HANDLERY (bez zmian) ===

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
    session.clear();
    Redirect::to("/")
}

async fn dashboard() -> &'static str {
    "Witaj w panelu admina! Jesteś zalogowany."
}

// === MIDDLEWARE ("STRAŻNIK") ===
pub async fn auth_middleware(
    session: Session,
    // Pobieramy stan, aby Axum wiedział, że middleware go wymaga.
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

// Trasy publiczne (logowanie)
pub fn public_admin_routes() -> Router<AppState> {
    Router::new().route("/login", get(get_login_form).post(post_login))
}

// Trasy chronione (panel, wylogowanie)
pub fn protected_admin_routes() -> Router<AppState> {
    Router::new()
        .route("/dashboard", get(dashboard))
        .route("/logout", get(logout))
}
