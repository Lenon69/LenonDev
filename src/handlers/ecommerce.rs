// src/handlers/ecommerce.rs
use crate::{
    appstate::{AppState, CacheValue},
    components::{ecommerce, layout},
};
use axum::{
    extract::State,
    http::{HeaderMap, Uri},
    response::Html,
};
use maud::html;

pub async fn get_ecommerce_page(
    uri: Uri,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> CacheValue {
    let cache_key = "page:/oferta/sklep-internetowy".to_string();
    let is_htmx_request = headers.contains_key("HX-Request");

    if !is_htmx_request {
        if let Some(cached_page) = state.cache.get(&cache_key) {
            return cached_page;
        }
    }

    let page_title = "Sklepy Internetowe | Sprzedawaj Skutecznie Online - LenonDev";
    let content_fragment = ecommerce::ecommerce_page_view();

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
            "Kompletne rozwiązania e-commerce, które pozwolą Ci sprzedawać skutecznie w internecie.",
        ),
        None,
        uri.path(),
    ));

    let response = (HeaderMap::new(), full_page_html);
    state.cache.insert(cache_key, response.clone());
    response
}
