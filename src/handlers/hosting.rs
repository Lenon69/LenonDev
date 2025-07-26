// src/handlers/hosting.rs
use crate::appstate::CacheValue;
use crate::{
    appstate::AppState,
    components::{hosting, layout},
};
use axum::{
    extract::State,
    http::{HeaderMap, Uri},
    response::Html,
};

pub async fn get_hosting_page(
    uri: Uri,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> CacheValue {
    let cache_key = "page:/oferta/hosting".to_string();
    if let Some(cached_page) = state.cache.get(&cache_key) {
        return cached_page;
    }

    let content_fragment = hosting::hosting_page_view();
    if headers.contains_key("HX-Request") {
        return (HeaderMap::new(), Html(content_fragment));
    }

    let full_page_html = Html(layout::base_layout(
        "Hosting i Domena - LenonDev",
        content_fragment,
        Some(
            "Zapewnij swojej stronie solidny fundament. Pomoc w wyborze, konfiguracji i zarządzaniu.",
        ),
        None,
        uri.path(),
    ));

    let response = (HeaderMap::new(), full_page_html);
    state.cache.insert(cache_key, response.clone());
    response
}
