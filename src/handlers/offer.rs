use crate::models::{OfferCatalogSchema, OfferItem, OfferedService, PriceSpecification};
// src/handlers/offer.rs
use crate::AppState;
use crate::appstate::CacheValue;
use crate::components::{layout, offer};
use axum::extract::State;
use axum::{http::HeaderMap, response::Html};

// Handler, który serwuje stronę /oferta
pub async fn get_offer_page(headers: HeaderMap, State(state): State<AppState>) -> CacheValue {
    let cache_key = "page:/oferta".to_string();
    let is_htmx_request = headers.contains_key("HX-Request");

    if let Some(cached_page) = state.cache.get(&cache_key) {
        if is_htmx_request {
            return (HeaderMap::new(), cached_page.1);
        }
        return cached_page;
    }

    // --- Tworzenie danych strukturalnych ---
    let offers = vec![
        OfferItem {
            type_of: "Offer",
            item_offered: OfferedService {
                type_of: "Service",
                name: "Prosta Strona Wizytówka",
                description: "Idealna na start – profesjonalnie zaprezentuj swoją firmę, usługi i dane kontaktowe.",
            },
            price_specification: PriceSpecification {
                type_of: "PriceSpecification",
                min_price: "1500",
                max_price: "4000",
                price_currency: "PLN",
            },
        },
        OfferItem {
            type_of: "Offer",
            item_offered: OfferedService {
                type_of: "Service",
                name: "Sklep Internetowy",
                description: "Sprzedawaj swoje produkty online. Kompletne rozwiązanie e-commerce z płatnościami i zarządzaniem.",
            },
            price_specification: PriceSpecification {
                type_of: "PriceSpecification",
                min_price: "12000",
                max_price: "22000",
                price_currency: "PLN",
            },
        },
        OfferItem {
            type_of: "Offer",
            item_offered: OfferedService {
                type_of: "Service",
                name: "Landing Page",
                description: "Skupiona na jednym celu – idealna do kampanii marketingowych, promocji produktu lub zapisu na newsletter.",
            },
            price_specification: PriceSpecification {
                type_of: "PriceSpecification",
                min_price: "2000",
                max_price: "4000",
                price_currency: "PLN",
            },
        },
        OfferItem {
            type_of: "Offer",
            item_offered: OfferedService {
                type_of: "Service",
                name: "Strona z Blogiem / CMS",
                description: "Dziel się wiedzą i buduj pozycję eksperta. Prosty w obsłudze system do zarządzania treścią.",
            },
            price_specification: PriceSpecification {
                type_of: "PriceSpecification",
                min_price: "4500",
                max_price: "9000",
                price_currency: "PLN",
            },
        },
        OfferItem {
            type_of: "Offer",
            item_offered: OfferedService {
                type_of: "Service",
                name: "Projekt Indywidualny",
                description: "Masz unikalny pomysł? Stworzę dedykowaną aplikację webową idealnie dopasowaną do Twoich potrzeb.",
            },
            price_specification: PriceSpecification {
                type_of: "PriceSpecification",
                min_price: "5000",
                max_price: "50000",
                price_currency: "PLN",
            },
        },
        OfferItem {
            type_of: "Offer",
            item_offered: OfferedService {
                type_of: "Service",
                name: "Aplikacja webowa (CRM)",
                description: "Zaawansowana logika backendowa.",
            },
            price_specification: PriceSpecification {
                type_of: "PriceSpecification",
                min_price: "15000",
                max_price: "50000",
                price_currency: "PLN",
            },
        },
    ];

    let schema = OfferCatalogSchema {
        context: "https://schema.org",
        type_of: "OfferCatalog",
        name: "Oferta Usług Web Developmentu",
        item_list_element: offers,
    };

    let schema_json = serde_json::to_string(&schema).unwrap_or_default();
    // ------------------------------------

    let content_fragment = offer::offer_page_view();
    let page_html = if is_htmx_request {
        Html(content_fragment)
    } else {
        Html(layout::base_layout(
            "LenonDev - Oferta",
            content_fragment,
            Some(
                "Nowoczesne rozwiązania webowe, które pomogą Twojej firmie zaistnieć w internecie i osiągnąć sukces.",
            ),
            Some(schema_json), // Przekazujemy dane strukturalne
        ))
    };

    let response = (HeaderMap::new(), page_html);
    state.cache.insert(cache_key, response.clone());
    response
}
