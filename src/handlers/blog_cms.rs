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
use maud::html;

pub async fn get_blog_cms_page(
    uri: Uri,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> CacheValue {
    let cache_key = "page:/oferta/blog-cms".to_string();
    let is_htmx_request = headers.contains_key("HX-Request");

    if !is_htmx_request {
        if let Some(cached_page) = state.cache.get(&cache_key) {
            return cached_page;
        }
    }

    let page_title = "Strona z Blogiem i CMS | Dziel Się Wiedzą - LenonDev";
    let content_fragment = blog_cms::blog_cms_page_view();

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
            "Dziel się wiedzą i buduj autorytet dzięki stronie z prostym w obsłudze systemem zarządzania treścią.",
        ),
        None,
        uri.path(),
    ));

    let response = (HeaderMap::new(), full_page_html);
    state.cache.insert(cache_key, response.clone());
    response
}
