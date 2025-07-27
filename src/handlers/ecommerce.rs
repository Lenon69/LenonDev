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

pub async fn get_ecommerce_page(
    uri: Uri,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> CacheValue {
    let cache_key = "page:/oferta/sklep-internetowy".to_string();
    if let Some(cached_page) = state.cache.get(&cache_key) {
        return cached_page;
    }

    let content_fragment = ecommerce::ecommerce_page_view();

    if headers.contains_key("HX-Request") {
        return (HeaderMap::new(), Html(content_fragment));
    }

    let full_page_html = Html(layout::base_layout(
        "Sklep Internetowy - LenonDev",
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
