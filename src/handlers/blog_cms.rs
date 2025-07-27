// src/handlers/blog_cms.rs
use crate::{
    appstate::{AppState, CacheValue},
    components::{blog_cms, layout},
};
use axum::{
    extract::State,
    http::{HeaderMap, Uri},
    response::Html,
};

pub async fn get_blog_cms_page(
    uri: Uri,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> CacheValue {
    let cache_key = "page:/oferta/blog-cms".to_string();
    if let Some(cached_page) = state.cache.get(&cache_key) {
        return cached_page;
    }

    let content_fragment = blog_cms::blog_cms_page_view();

    if headers.contains_key("HX-Request") {
        return (HeaderMap::new(), Html(content_fragment));
    }

    let full_page_html = Html(layout::base_layout(
        "Strona z Blogiem / CMS - LenonDev",
        content_fragment,
        Some(
            "Dziel się wiedzą i buduj autorytet dzięki stronie z prostym w obsłudze systemem zarządzania treścią.",
        ),
        None,
        uri.path(),
    ));

    let response = (HeaderMap::new(), full_page_html);
    state.cache.insert(cache_key, response.clone());
    response
}
