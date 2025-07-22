// src/handlers/blog.rs
use crate::{
    AppState,
    appstate::CacheValue,
    components::{blog, layout},
    models::{Article, ArticleSchema, Author, ImageObject, Publisher},
};
use axum::{
    extract::{Path, State},
    http::{HeaderMap, Uri},
    response::{Html, IntoResponse},
};
use maud::{PreEscaped, html};

// Handler dla /blog
pub async fn blog_index(uri: Uri, headers: HeaderMap, State(state): State<AppState>) -> CacheValue {
    let cache_key = "page:/blog".to_string();
    let is_htmx_request = headers.contains_key("HX-Request");

    // Jeśli to NIE jest zapytanie HTMX, sprawdzamy cache
    if !is_htmx_request {
        if let Some(cached_page) = state.cache.get(&cache_key) {
            return cached_page;
        }
    }

    // Generujemy treść (zawsze, bo HTMX omija cache, a pełne przeładowanie nie znalazło nic w cache'u)
    let articles = sqlx::query_as::<_, Article>(
        "SELECT * FROM articles WHERE published_at IS NOT NULL ORDER BY published_at DESC",
    )
    .fetch_all(&state.db_pool)
    .await
    .unwrap_or_else(|_| vec![]);

    let content_fragment = blog::blog_index_view(articles);

    // Jeśli to zapytanie HTMX, zwróć tylko fragment opakowany w CacheValue
    if is_htmx_request {
        return (HeaderMap::new(), Html(content_fragment));
    }

    // W przeciwnym razie, zbuduj całą stronę
    let full_page_html = Html(layout::base_layout(
        "LenonDev - Blog",
        content_fragment,
        Some(
            "Blog o nowoczesnym web developmencie, technologii Rust, Axum, HTMX i tworzeniu wydajnych aplikacji internetowych.",
        ),
        None,
        uri.path(),
    ));

    // Stwórz odpowiedź, zapisz ją w cache'u i zwróć
    let response = (HeaderMap::new(), full_page_html);
    state.cache.insert(cache_key, response.clone());

    response
}

// Handler dla /blog/{slug} - W pełni przepisany i zoptymalizowany
pub async fn show_article(
    uri: Uri,
    headers: HeaderMap,
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> impl IntoResponse {
    let is_htmx_request = headers.contains_key("HX-Request");

    // Jeśli to NIE JEST zapytanie HTMX, sprawdzamy cache dla PEŁNEJ STRONY
    if !is_htmx_request {
        let cache_key = format!("page:/blog/{}", slug);
        if let Some(cached_page) = state.cache.get(&cache_key) {
            return cached_page.into_response();
        }
    }

    // Jeśli to zapytanie HTMX lub w cache'u nie ma strony, generujemy treść
    let article_result = sqlx::query_as::<_, Article>(
        "SELECT * FROM articles WHERE slug = $1 AND published_at IS NOT NULL",
    )
    .bind(&slug)
    .fetch_one(&state.db_pool)
    .await;

    match article_result {
        Ok(article) => {
            // Generowanie fragmentu HTML
            let sections: Vec<String> = article
                .content
                .split("---")
                .filter(|s| !s.trim().is_empty())
                .map(|section_md| {
                    let parser = pulldown_cmark::Parser::new(section_md.trim());
                    let mut html_output = String::new();
                    pulldown_cmark::html::push_html(&mut html_output, parser);
                    html_output
                })
                .collect();

            let content_fragment = html! {
                div class="container mx-auto px-4 pb-16 lg:pb-24 pt-36 md:pt-28" {
                    div class="max-w-4xl mx-auto" {
                        div class="text-center mb-12" {
                            @if let Some(published_at) = article.published_at {
                                p class="text-sm text-slate-400 mb-4" { (published_at.format("%d %B %Y")) }
                            }
                            h1 class="text-4xl lg:text-6xl font-bold tracking-tighter bg-gradient-to-r from-brand-cyan to-brand-green text-transparent bg-clip-text py-4" {
                                (article.title)
                            }
                        }
                        @for rendered_html in &sections {
                            section class="py-10" {
                                div class="prose prose-invert prose-xl" { (PreEscaped(rendered_html)) }
                            }
                        }
                        div class="text-center mt-16" {
                            a hx-get="/blog" hx-target="#content-area" hx-push-url="/blog" class="cursor-pointer inline-block bg-slate-700 hover:bg-slate-600 transition-colors text-white font-bold py-2 px-6 rounded-lg" { "← Wróć na bloga" }
                        }
                    }
                }
            };

            // Jeśli to zapytanie HTMX, zwróć tylko fragment
            if is_htmx_request {
                return Html(content_fragment).into_response();
            }

            // W przeciwnym razie, zbuduj całą stronę i zapisz ją w cache'u
            let base_url = std::env::var("APP_BASE_URL").unwrap_or_default();
            let og_image_url = format!("{}/public/og-image.png", base_url);
            let schema = ArticleSchema {
                context: "https://schema.org",
                type_of: "BlogPosting",
                headline: article.title.clone(),
                image: ImageObject {
                    type_of: "ImageObject",
                    url: og_image_url,
                    width: 1200,
                    height: 630,
                },
                author: Author {
                    type_of: "Person".to_string(),
                    name: "Lenon".to_string(),
                },
                date_published: article
                    .published_at
                    .map(|d| d.to_rfc3339())
                    .unwrap_or_default(),
                publisher: Publisher {
                    type_of: "Organization",
                    name: "LenonDev",
                    logo: ImageObject {
                        type_of: "ImageObject",
                        url: format!("{}/public/fixed-logo.png", base_url),
                        width: 372,
                        height: 281,
                    },
                },
                date_modified: article.updated_at.to_rfc3339(),
            };
            let schema_json = serde_json::to_string(&schema).ok();

            let full_page_html = Html(layout::base_layout(
                &article.title,
                content_fragment,
                article.excerpt.as_deref(),
                schema_json,
                &format!("/blog/{}", slug),
            ));

            // Zapisz PEŁNĄ stronę w cache'u
            let cache_key = format!("page:/blog/{}", slug);
            state
                .cache
                .insert(cache_key, (HeaderMap::new(), full_page_html.clone()));

            full_page_html.into_response()
        }
        Err(_) => {
            // Tworzymy fragment HTML z informacją o błędzie 404
            let error_content = html! {
                div class="container mx-auto px-4 pb-16 lg:pb-24 pt-36 md:pt-28" {
                    div class="text-center py-40" {
                        h1 class="text-2xl text-red-500" { "404 - Nie znaleziono artykułu" }
                        a hx-get="/blog" hx-target="#content-area" hx-push-url="/blog" class="mt-8 cursor-pointer inline-block bg-slate-700 hover:bg-slate-600 transition-colors text-white font-bold py-2 px-6 rounded-lg" {
                            "← Wróć na bloga"
                        }
                    }
                }
            };

            // Jeśli to zapytanie HTMX, zwróć tylko sam fragment błędu
            if is_htmx_request {
                return Html(error_content).into_response();
            }

            // W przeciwnym razie, zwróć pełną stronę błędu 404 z całym layoutem
            Html(layout::base_layout(
                "404 - Nie znaleziono",
                error_content,
                Some("Strona, której szukasz, nie została znaleziona."),
                None,
                uri.path(),
            ))
            .into_response()
        }
    }
}
