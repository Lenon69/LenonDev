// src/handlers/blog.rs
use crate::{
    AppState,
    appstate::CacheValue,
    components::{blog, layout},
    models::{Article, ArticleSchema, Author},
};
use axum::{
    extract::{Path, State},
    http::HeaderMap,
    response::Html,
};
use maud::{PreEscaped, html};

// Handler dla /blog
pub async fn blog_index(headers: HeaderMap, State(state): State<AppState>) -> CacheValue {
    let cache_key = "page:/blog".to_string();

    // Sprawdzamy cache
    if let Some(cached_page) = state.cache.get(&cache_key) {
        return cached_page;
    }

    let articles = sqlx::query_as::<_, Article>(
        "SELECT * FROM articles WHERE published_at IS NOT NULL ORDER BY published_at DESC",
    )
    .fetch_all(&state.db_pool)
    .await
    .unwrap_or_else(|_| vec![]);

    let content_fragment = blog::blog_index_view(articles);

    let page_html = if headers.contains_key("HX-Request") {
        Html(content_fragment)
    } else {
        Html(layout::base_layout(
            "LenonDev - Blog",
            content_fragment,
            None,
            None,
        ))
    };

    // Zapisujemy stronę w cache'u i ją zwracamy
    let response = (HeaderMap::new(), page_html);
    state.cache.insert(cache_key, response.clone());

    response
}

// Handler dla /blog/{slug} - W pełni przepisany i zoptymalizowany
pub async fn show_article(
    headers: HeaderMap,
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> CacheValue {
    // Tworzymy dynamiczny klucz dla cache'a
    let cache_key = format!("page:/blog/{}", slug);
    let is_htmx_request = headers.contains_key("HX-Request");

    // Sprawdzamy cache
    if let Some(cached_page) = state.cache.get(&cache_key) {
        // Jeśli to zapytanie HTMX, zwróć tylko drugi element krotki (sam HTML)
        if is_htmx_request {
            return (HeaderMap::new(), cached_page.1);
        }
        return cached_page;
    }

    // Jeśli nie ma w cache'u, pobierz artykuł z bazy
    let article_result = sqlx::query_as::<_, Article>(
        "SELECT * FROM articles WHERE slug = $1 AND published_at IS NOT NULL",
    )
    .bind(&slug)
    .fetch_one(&state.db_pool)
    .await;

    // Przekształcamy wynik w odpowiedź HTML
    match article_result {
        Ok(article) => {
            // Przetwarzamy Markdown na HTML
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

            // Renderujemy główną treść artykułu
            let content_fragment = html! {
                div class="container mx-auto px-4 pb-16 lg:pb-24" {
                    div class="max-w-4xl mx-auto" {
                        div class="text-center mb-12" {
                            @if let Some(published_at) = article.published_at {
                                p class="text-sm text-slate-400 mb-4" { (published_at.format("%d %B %Y")) }
                            }
                            h1 class="text-4xl lg:text-6xl font-bold tracking-tighter bg-gradient-to-r from-brand-cyan to-brand-green text-transparent bg-clip-text" {
                                (article.title)
                            }
                        }

                        @for rendered_html in &sections {
                            section class="py-10" {
                                div class="prose prose-invert prose-xl" {
                                    (PreEscaped(rendered_html))
                                }
                            }
                        }

                        div class="text-center mt-16" {
                            a hx-get="/blog" hx-target="#content-area" hx-push-url="/blog" class="cursor-pointer inline-block bg-slate-700 hover:bg-slate-600 transition-colors text-white font-bold py-2 px-6 rounded-lg" {
                                "← Wróć na bloga"
                            }
                        }
                    }
                }
            };

            let schema = ArticleSchema {
                context: "https://schema.org".to_string(),
                type_of: "BlogPosting".to_string(),
                headline: article.title.clone(),
                author: Author {
                    type_of: "Person".to_string(),
                    name: "Lenon".to_string(),
                },
            };
            let schema_json = serde_json::to_string(&schema).unwrap_or_default();

            // Tworzymy pełną odpowiedź (z layoutem lub bez)
            let page_html = if is_htmx_request {
                Html(content_fragment)
            } else {
                Html(layout::base_layout(
                    &article.title,
                    content_fragment,
                    article.excerpt.as_deref(),
                    Some(schema_json),
                ))
            };

            // Zapisujemy pełną stronę do cache'a i ją zwracamy
            let response = (HeaderMap::new(), page_html);
            state.cache.insert(cache_key, response.clone());
            response
        }
        Err(_) => {
            // Artykułu nie znaleziono - zwracamy stronę błędu 404
            let error_content = html! {
                div class="text-center py-40" {
                    h1 class="text-2xl text-red-500" { "404 - Nie znaleziono artykułu" }
                }
            };
            let page_html = if is_htmx_request {
                Html(error_content)
            } else {
                Html(layout::base_layout(
                    "404 - Nie znaleziono",
                    error_content,
                    None,
                    None,
                ))
            };
            (HeaderMap::new(), page_html)
        }
    }
}
