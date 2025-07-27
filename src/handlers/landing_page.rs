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
use maud::html;

pub async fn get_landing_page(
    uri: Uri,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> CacheValue {
    let cache_key = "page:/oferta/landing-page".to_string();
    let is_htmx_request = headers.contains_key("HX-Request");

    if !is_htmx_request {
        if let Some(cached_page) = state.cache.get(&cache_key) {
            return cached_page;
        }
    }

    let page_title = "Landing Page | Strony Koncentrujące się na Konwersji - LenonDev";
    let content_fragment = landing_page::landing_page_view();

    if is_htmx_request {
        let htmx_response = html! {
            title hx-swap-oob="true" { (page_title) }
            (content_fragment)
        };
        return (HeaderMap::new(), Html(htmx_response));
    }

    let full_page_html = Html(layout::base_layout(
        page_title,
        content_fragment,
        Some("Strony typu landing page zaprojektowane w jednym celu: maksymalizacji konwersji."),
        None,
        uri.path(),
    ));

    let response = (HeaderMap::new(), full_page_html);
    state.cache.insert(cache_key, response.clone());
    response
}
