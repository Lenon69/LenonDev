// src/handlers/seo_optimization.rs
use crate::appstate::CacheValue;
use crate::{
    appstate::AppState,
    components::{layout, seo_optimization},
};
use axum::{
    extract::State,
    http::{HeaderMap, Uri},
    response::Html,
};

pub async fn get_seo_optimization_page(
    uri: Uri,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> CacheValue {
    let cache_key = "page:/oferta/seo".to_string();
    if let Some(cached_page) = state.cache.get(&cache_key) {
        return cached_page;
    }

    let content_fragment = seo_optimization::seo_optimization_page_view();
    if headers.contains_key("HX-Request") {
        return (HeaderMap::new(), Html(content_fragment));
    }

    let full_page_html = Html(layout::base_layout(
        "Optymalizacja i SEO - LenonDev",
        content_fragment,
        Some(
            "Zwiększ widoczność i dotrzyj do nowych klientów dzięki profesjonalnej optymalizacji SEO.",
        ),
        None,
        uri.path(),
    ));

    let response = (HeaderMap::new(), full_page_html);
    state.cache.insert(cache_key, response.clone());
    response
}
