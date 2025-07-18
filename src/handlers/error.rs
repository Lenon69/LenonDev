// W nowym pliku src/handlers/error.rs
use crate::components::layout;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

pub async fn handler_404() -> impl IntoResponse {
    let content = maud::html! {
        div class="text-center py-40" {
            h1 class="text-6xl font-bold text-brand-cyan" { "404" }
            p class="text-2xl text-slate-300 mt-4" { "Strona nie została znaleziona." }
            a href="/" class="mt-8 inline-block bg-brand-purple hover:opacity-80 transition-opacity text-white font-bold py-3 px-6 rounded-lg" {
                "Wróć na stronę główną"
            }
        }
    };
    (
        StatusCode::NOT_FOUND,
        Html(layout::base_layout("404 - Nie znaleziono", content)),
    )
}
