// src/handlers/web_app.rs
use crate::{
    appstate::{AppState, CacheValue},
    components::{layout, web_app},
};
use axum::{
    extract::State,
    http::{HeaderMap, Uri},
    response::Html,
};

pub async fn get_web_app_page(
    uri: Uri,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> CacheValue {
    let cache_key = "page:/oferta/aplikacja-webowa-crm".to_string();
    if let Some(cached_page) = state.cache.get(&cache_key) {
        return cached_page;
    }

    let content_fragment = web_app::web_app_page_view();

    if headers.contains_key("HX-Request") {
        return (HeaderMap::new(), Html(content_fragment));
    }

    let full_page_html = Html(layout::base_layout(
        "Aplikacje Webowe - LenonDev",
        content_fragment,
        Some(
            "Zaawansowane aplikacje webowe (CRM, systemy rezerwacyjne), które automatyzują procesy w Twojej firmie.",
        ),
        None,
        uri.path(),
    ));

    let response = (HeaderMap::new(), full_page_html);
    state.cache.insert(cache_key, response.clone());
    response
}
