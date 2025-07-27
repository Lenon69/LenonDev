// src/handlers/maintenance.rs
use crate::{
    appstate::{AppState, CacheValue},
    components::{layout, maintenance},
};
use axum::{
    extract::State,
    http::{HeaderMap, Uri},
    response::Html,
};
use maud::html;

pub async fn get_maintenance_page(
    uri: Uri,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> CacheValue {
    let cache_key = "page:/oferta/opieka".to_string();
    let is_htmx_request = headers.contains_key("HX-Request");

    if !is_htmx_request {
        if let Some(cached_page) = state.cache.get(&cache_key) {
            return cached_page;
        }
    }

    let page_title = "Opieka Techniczna - LenonDev"; // Definiujemy tytuł
    let content_fragment = maintenance::maintenance_page_view();

    // ZMIANA TUTAJ: Kompleksowa obsługa żądań HTMX
    if is_htmx_request {
        // Dla HTMX, wysyłamy treść ORAZ tytuł do podmiany "poza pasmem"
        let htmx_response = html! {
            // Ten tag zostanie wstawiony do <head> dzięki `hx-swap-oob`
            title hx-swap-oob="true" { (page_title) }
            // To jest właściwa treść, która trafi do #content-area
            (content_fragment)
        };
        return (HeaderMap::new(), Html(htmx_response));
    }

    // Dla pełnego ładowania strony (np. odświeżenie) budujemy cały layout
    let full_page_html = Html(layout::base_layout(
        page_title,
        content_fragment,
        Some("Pakiety opieki i wsparcia technicznego dla Twojej strony internetowej."),
        None,
        uri.path(),
    ));

    let response = (HeaderMap::new(), full_page_html);
    state.cache.insert(cache_key, response.clone());
    response
}
