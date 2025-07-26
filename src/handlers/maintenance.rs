// src/handlers/maintenance.rs
use crate::{
    appstate::{AppState, CacheValue},
    components::{layout, maintenance},
};
use axum::{
    extract::State,
    http::{HeaderMap, Uri},
    response::Html,
};

pub async fn get_maintenance_page(
    uri: Uri,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> CacheValue {
    let cache_key = "page:/oferta/opieka".to_string();

    if let Some(cached_page) = state.cache.get(&cache_key) {
        return cached_page;
    }

    let content_fragment = maintenance::maintenance_page_view();

    if headers.contains_key("HX-Request") {
        return (HeaderMap::new(), Html(content_fragment));
    }

    let full_page_html = Html(layout::base_layout(
        "Opieka Techniczna - LenonDev",
        content_fragment,
        Some("Pakiety opieki i wsparcia technicznego dla Twojej strony internetowej."),
        None,
        uri.path(),
    ));

    let response = (HeaderMap::new(), full_page_html);
    state.cache.insert(cache_key, response.clone());
    response
}
