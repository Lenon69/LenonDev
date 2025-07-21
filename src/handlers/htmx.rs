// src/handlers/htmx.rs
use crate::{AppState, appstate::CacheValue, components::sections, models::Project};
use axum::{
    extract::{Query, State},
    http::HeaderMap,
};
use maud::html;
use serde::Deserialize;

// Struktura do odczytania parametru ?scroll_to=...
#[derive(Deserialize)]
pub struct ScrollParams {
    pub scroll_to: Option<String>,
}

pub async fn get_main_content(
    State(state): State<AppState>,
    Query(params): Query<ScrollParams>, // Odczytujemy parametry z URL
) -> CacheValue {
    let cache_key = format!(
        "page:/:scroll_to={}",
        params.scroll_to.as_deref().unwrap_or("")
    );

    // Sprawdzamy, czy strona jest w cache'u
    if let Some(cached_response) = state.cache.get(&cache_key) {
        return cached_response;
    }

    // Pobieramy projekty, tak jak wcześniej
    let projects = sqlx::query_as::<_, Project>("SELECT id, title, slug, description, technologies, image_url, project_url FROM projects ORDER BY id ASC")
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
        let trigger_value = format!("{{\"scrollToSection\": \"#{}\"}}", section_id);
        if let Ok(header_value) = trigger_value.parse() {
            headers.insert("HX-Trigger", header_value);
        }
    }

    let response: CacheValue = (headers, markup.into());
    state.cache.insert(cache_key, response.clone());

    response
}
