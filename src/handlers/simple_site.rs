// src/handlers/simple_site.rs
use crate::{
    appstate::{AppState, CacheValue},
    components::{layout, simple_site},
};
use axum::{
    extract::State,
    http::{HeaderMap, Uri},
    response::Html,
};
use maud::html;

pub async fn get_simple_site_page(
    uri: Uri,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> CacheValue {
    let cache_key = "page:/oferta/prosta-strona-wizytowka".to_string();
    let is_htmx_request = headers.contains_key("HX-Request");

    if !is_htmx_request {
        if let Some(cached_page) = state.cache.get(&cache_key) {
            return cached_page;
        }
    }

    let page_title = "Strona Wizytówka | Profesjonalny Wizerunek w Sieci - LenonDev";
    let content_fragment = simple_site::simple_site_page_view();

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
        Some("Profesjonalna strona-wizytówka to fundament obecności w internecie."),
        None,
        uri.path(),
    ));

    let response = (HeaderMap::new(), full_page_html);
    state.cache.insert(cache_key, response.clone());
    response
}
