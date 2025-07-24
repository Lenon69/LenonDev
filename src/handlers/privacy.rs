// src/handlers/privacy.rs
use crate::{
    appstate::{AppState, CacheValue},
    components::{layout, privacy},
};
use axum::{
    extract::State,
    http::{HeaderMap, Uri},
    response::Html,
};

pub async fn get_privacy_policy_page(
    uri: Uri,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> CacheValue {
    let cache_key = "page:/polityka-prywatnosci".to_string();

    if !headers.contains_key("HX-Request") {
        if let Some(cached_page) = state.cache.get(&cache_key) {
            return cached_page;
        }
    }

    let content_fragment = privacy::privacy_policy_page();

    if headers.contains_key("HX-Request") {
        return (HeaderMap::new(), Html(content_fragment));
    }

    let full_page_html = Html(layout::base_layout(
        "Polityka Cookies - LenonDev",
        content_fragment,
        Some("Polityka dotycząca plików cookies na stronie LenonDev."),
        None,
        uri.path(),
    ));

    let response = (HeaderMap::new(), full_page_html);
    state.cache.insert(cache_key, response.clone());
    response
}
