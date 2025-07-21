use std::sync::OnceLock;

// src/handlers/htmx.rs
use crate::{AppState, components::sections, models::Project};
use axum::{
    extract::{Query, State},
    http::HeaderMap,
    response::{Html, IntoResponse},
};
use maud::{Markup, html};
use moka::sync::Cache;
use serde::Deserialize;

// Struktura do odczytania parametru ?scroll_to=...
#[derive(Deserialize)]
pub struct ScrollParams {
    scroll_to: Option<String>,
}

// Globalny cache dla renderowanych stron
static RENDER_CACHE: OnceLock<Cache<String, (HeaderMap, Html<Markup>)>> = OnceLock::new();

fn get_cache() -> &'static Cache<String, (HeaderMap, Html<Markup>)> {
    RENDER_CACHE.get_or_init(|| {
        Cache::builder()
            .max_capacity(100)
            .time_to_live(std::time::Duration::from_secs(3600 * 24))
            .build()
    })
}

pub async fn get_main_content(
    State(state): State<AppState>,
    Query(params): Query<ScrollParams>, // Odczytujemy parametry z URL
) -> impl IntoResponse {
    let cache = get_cache();
    // Klucz dla strony głównej, uwzględniający parametr scroll_to
    let cache_key = format!(
        "page:/:scroll_to={}",
        params.scroll_to.as_deref().unwrap_or("")
    );

    // Sprawdzamy, czy strona jest w cache'u
    if let Some(cached_response) = cache.get(&cache_key) {
        return (cached_response.0, cached_response.1);
    }

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
        let trigger_value = format!("{{\"scrollToSection\": \"#{}\"}}", section_id);
        if let Ok(header_value) = trigger_value.parse() {
            headers.insert("HX-Trigger", header_value);
        }
    }

    // Tworzymy odpowiedź i zapisujemy ją w cache'u
    let response = (headers, Html(markup));
    cache.insert(cache_key, response.clone());

    response
}
