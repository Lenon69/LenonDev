use std::sync::OnceLock;

// src/handlers/offer.rs
use crate::components::{layout, offer};
use axum::{http::HeaderMap, response::Html};
use maud::Markup;
use moka::sync::Cache;

// Globalny cache dla renderowanych stron
static RENDER_CACHE: OnceLock<Cache<String, Html<Markup>>> = OnceLock::new();

fn get_cache() -> &'static Cache<String, Html<Markup>> {
    RENDER_CACHE.get_or_init(|| {
        // Cache przechowuje do 100 stron przez 1 godzinę
        Cache::builder()
            .max_capacity(100)
            .time_to_live(std::time::Duration::from_secs(3600 * 24))
            .build()
    })
}

// Handler, który serwuje stronę /oferta
pub async fn get_offer_page(headers: HeaderMap) -> Html<Markup> {
    let cache = get_cache();
    let cache_key = "page:/oferta".to_string();

    // Sprawdź, czy strona jest w cache'u
    if let Some(cached_page) = cache.get(&cache_key) {
        return cached_page;
    }

    // Jeśli nie ma w cache'u, wygeneruj stronę
    let content_fragment = offer::offer_page_view();
    let full_page = if headers.contains_key("HX-Request") {
        Html(content_fragment)
    } else {
        Html(layout::base_layout("LenonDev - Oferta", content_fragment))
    };

    // Zapisz wygenerowaną stronę do cache'a i ją zwróć
    cache.insert(cache_key, full_page.clone());
    full_page
}
