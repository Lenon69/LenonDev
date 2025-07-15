// src/handlers/blog.rs
use crate::{
    AppState,
    components::{blog, layout},
    models::Article,
};
use axum::{
    extract::{Path, State},
    http::HeaderMap,
    response::Html,
};
use maud::{Markup, PreEscaped, html};

// Handler dla /blog
pub async fn blog_index(headers: HeaderMap, State(state): State<AppState>) -> Html<Markup> {
    let articles = sqlx::query_as::<_, Article>(
        "SELECT * FROM articles WHERE published_at IS NOT NULL ORDER BY published_at DESC",
    )
    .fetch_all(&state.db_pool)
    .await
    .unwrap_or_else(|_| vec![]);

    let content_fragment = blog::blog_index_view(articles);

    if headers.contains_key("HX-Request") {
        Html(content_fragment)
    } else {
        Html(layout::base_layout("LenonDev - Blog", content_fragment))
    }
}

// Handler dla /blog/{slug}
pub async fn show_article(
    headers: HeaderMap,
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Html<Markup> {
    let article_content = if let Ok(article) = sqlx::query_as::<_, Article>(
        "SELECT * FROM articles WHERE slug = $1 AND published_at IS NOT NULL",
    )
    .bind(&slug) // Przekazujemy slug jako referencję
    .fetch_one(&state.db_pool)
    .await
    {
        // Logika dzielenia na sekcje
        let html_sections: Vec<String> = article
            .content
            .split("---")
            .filter(|s| !s.trim().is_empty())
            .map(|section_md| {
                let parser = pulldown_cmark::Parser::new(section_md);
                let mut html_output = String::new();
                pulldown_cmark::html::push_html(&mut html_output, parser);
                html_output
            })
            .collect();

        // Generowanie fragmentu dla pojedynczego artykułu
        let title = article.title.clone();
        let content_fragment = html! {
            div class="container mx-auto px-4 py-16 lg:py-24" {
                div class="max-w-4xl mx-auto" {
                    // Sekcja tytułowa
                    div class="text-center mb-12" {
                        @if let Some(published_at) = article.published_at {
                            p class="text-sm text-slate-400 mb-4" { (published_at.format("%d %B %Y")) }
                        }
                        h1 class="text-4xl lg:text-6xl font-bold tracking-tighter bg-gradient-to-r from-brand-cyan to-brand-green text-transparent bg-clip-text" {
                            (title)
                        }
                    }
                    // Renderowanie sekcji
                    @for section_html in &html_sections {
                        section class="py-10" {
                            div class="prose prose-invert prose-xl prose-p:leading-loose
                                      prose-headings:text-brand-cyan prose-headings:mb-6
                                      prose-a:text-brand-green prose-a:no-underline hover:prose-a:underline
                                      prose-strong:text-slate-100
                                      prose-ul:list-none prose-ul:p-0
                                      prose-li:my-3 prose-li:pl-8 prose-li:relative
                                      prose-li:before:content-[''] prose-li:before:absolute prose-li:before:left-0 prose-li:before:top-2
                                      prose-li:before:h-4 prose-li:before:w-4
                                      prose-li:before:bg-[url('/svg/checkmark.svg')] prose-li:before:bg-contain" {
                                (PreEscaped(section_html))
                            }
                        }
                    }
                    // Przycisk powrotny
                    div class="text-center mt-16" {
                        a hx-get="/blog" hx-target="#content-area" hx-push-url="/blog" class="cursor-pointer inline-block bg-slate-700 hover:bg-slate-600 transition-colors text-white font-bold py-2 px-6 rounded-lg" {
                            "← Wróć na bloga"
                        }
                    }
                }
            }
        };
        (title, content_fragment)
    } else {
        // Obsługa błędu 404
        (
            "Nie znaleziono".to_string(),
            html! {
                div class="text-center py-40" {
                    h1 class="text-2xl text-red-500" { "404 - Nie znaleziono artykułu" }
                }
            },
        )
    };

    if headers.contains_key("HX-Request") {
        Html(article_content.1)
    } else {
        Html(layout::base_layout(&article_content.0, article_content.1))
    }
}
