// src/handlers/offer.rs
use crate::AppState;
use crate::appstate::CacheValue;
use crate::components::{layout, offer};
use axum::extract::State;
use axum::{http::HeaderMap, response::Html};

// Handler, który serwuje stronę /oferta
pub async fn get_offer_page(headers: HeaderMap, State(state): State<AppState>) -> CacheValue {
    let cache_key = "page:/oferta".to_string();

    // Sprawdź, czy strona jest w cache'u
    if let Some(cached_page) = state.cache.get(&cache_key) {
        return cached_page;
    }

    // Jeśli nie ma w cache'u, wygeneruj stronę
    let content_fragment = offer::offer_page_view();
    let page_html = if headers.contains_key("HX-Request") {
        Html(content_fragment)
    } else {
        Html(layout::base_layout(
            "LenonDev - Oferta",
            content_fragment,
            None,
            None,
        ))
    };

    // Zapisz wygenerowaną stronę do cache'a i ją zwróć
    let response = (HeaderMap::new(), page_html);
    state.cache.insert(cache_key, response.clone());
    response
}
