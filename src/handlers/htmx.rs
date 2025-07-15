// src/handlers/htmx.rs
use crate::{AppState, components::sections, models::Project};
use axum::{
    extract::{Query, State},
    http::HeaderMap,
    response::{Html, IntoResponse},
};
use maud::html;
use serde::Deserialize;

// Struktura do odczytania parametru ?scroll_to=...
#[derive(Deserialize)]
pub struct ScrollParams {
    scroll_to: Option<String>,
}

pub async fn get_main_content(
    State(state): State<AppState>,
    Query(params): Query<ScrollParams>, // Odczytujemy parametry z URL
) -> impl IntoResponse {
    // Pobieramy projekty, tak jak wcześniej
    let projects = sqlx::query_as::<_, Project>("SELECT * FROM projects")
        .fetch_all(&state.db_pool)
        .await
        .unwrap_or_else(|e| {
            println!("Failed to fetch projects: {}", e);
            vec![]
        });

    // Generujemy kod HTML
    let markup = html! {
        (sections::about_section())
        (sections::projects_section(projects))
        (sections::contact_section())
    };

    let mut headers = HeaderMap::new();

    // Jeśli w linku był parametr 'scroll_to'...
    if let Some(section_id) = params.scroll_to {
        // ...tworzymy i dodajemy nagłówek HX-Trigger, który wywoła zdarzenie po stronie klienta
        let trigger_value = format!("{{\"scrollToSection\": \"#{}\"}}", section_id);
        if let Ok(header_value) = trigger_value.parse() {
            headers.insert("HX-Trigger", header_value);
        }
    }

    // Zwracamy odpowiedź: nagłówki + kod HTML
    (headers, Html(markup))
}
