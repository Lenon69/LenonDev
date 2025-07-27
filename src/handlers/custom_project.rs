// src/handlers/custom_project.rs
use crate::{
    appstate::{AppState, CacheValue},
    components::{custom_project, layout},
};
use axum::{
    extract::State,
    http::{HeaderMap, Uri},
    response::Html,
};

pub async fn get_custom_project_page(
    uri: Uri,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> CacheValue {
    let cache_key = "page:/oferta/projekt-indywidualny".to_string();

    // Sprawdzamy, czy to żądanie HTMX
    let is_htmx_request = headers.contains_key("HX-Request");

    if !is_htmx_request {
        if let Some(cached_page) = state.cache.get(&cache_key) {
            return cached_page;
        }
    }

    let content_fragment = custom_project::custom_project_page_view();

    // ZMIANA TUTAJ: Zwracamy sam fragment dla HTMX
    if is_htmx_request {
        return (HeaderMap::new(), Html(content_fragment));
    }

    // Dla pełnego ładowania strony (np. po odświeżeniu) budujemy cały layout
    let full_page_html = Html(layout::base_layout(
        "Projekt Indywidualny - LenonDev",
        content_fragment,
        Some(
            "Masz unikalny pomysł na aplikację lub platformę? Stworzę rozwiązanie idealnie dopasowane do Twoich potrzeb.",
        ),
        None,
        uri.path(),
    ));

    let response = (HeaderMap::new(), full_page_html);
    state.cache.insert(cache_key, response.clone());
    response
}
