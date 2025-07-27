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
use maud::html;

pub async fn get_web_app_page(
    uri: Uri,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> CacheValue {
    let cache_key = "page:/oferta/aplikacja-webowa-crm".to_string();
    let is_htmx_request = headers.contains_key("HX-Request");

    if !is_htmx_request {
        if let Some(cached_page) = state.cache.get(&cache_key) {
            return cached_page;
        }
    }

    let page_title = "Aplikacje Webowe i CRM | Zautomatyzuj Swój Biznes - LenonDev";
    let content_fragment = web_app::web_app_page_view();

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
