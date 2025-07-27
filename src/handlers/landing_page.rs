// src/handlers/landing_page.rs
use crate::{
    appstate::{AppState, CacheValue},
    components::{landing_page, layout},
};
use axum::{
    extract::State,
    http::{HeaderMap, Uri},
    response::Html,
};

pub async fn get_landing_page(
    uri: Uri,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> CacheValue {
    let cache_key = "page:/oferta/landing-page".to_string();
    if let Some(cached_page) = state.cache.get(&cache_key) {
        return cached_page;
    }

    let content_fragment = landing_page::landing_page_view();

    if headers.contains_key("HX-Request") {
        return (HeaderMap::new(), Html(content_fragment));
    }

    let full_page_html = Html(layout::base_layout(
        "Landing Page - LenonDev",
        content_fragment,
        Some("Strony typu landing page zaprojektowane w jednym celu: maksymalizacji konwersji."),
        None,
        uri.path(),
    ));

    let response = (HeaderMap::new(), full_page_html);
    state.cache.insert(cache_key, response.clone());
    response
}
