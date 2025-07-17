// src/handlers/offer.rs
use crate::components::{layout, offer};
use axum::{http::HeaderMap, response::Html};
use maud::Markup;

// Handler, który serwuje stronę /oferta
pub async fn get_offer_page(headers: HeaderMap) -> Html<Markup> {
    // Generujemy treść (widok) strony z oferty
    let content_fragment = offer::offer_page_view();

    // Sprawdzamy, czy zapytanie pochodzi od HTMX, czy jest to pełne przeładowanie strony
    if headers.contains_key("HX-Request") {
        // Jeśli HTMX, zwróć tylko samą treść
        Html(content_fragment)
    } else {
        // Jeśli pełne przeładowanie, zwróć treść wewnątrz głównego szablonu
        Html(layout::base_layout("LenonDev - Oferta", content_fragment))
    }
}
