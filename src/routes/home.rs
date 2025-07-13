// src/routes/home.rs
use crate::components::{home_page, layout}; // Importujemy nasze komponenty
use axum::response::Html;
use maud::Markup;

// Funkcja jest teraz publiczna (`pub`)
pub async fn root_handler() -> Html<String> {
    // Składamy stronę z komponentów
    let page: Markup = layout::page(
        "LenonDev - Nowoczesne Aplikacje Internetowe",
        home_page::content(),
    );
    Html(page.into_string())
}
