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
    .bind(&slug)
    .fetch_one(&state.db_pool)
    .await
    {
        // Krok 1: Przygotowanie danych (bez zmian)
        let sections: Vec<(String, String)> = article
            .content
            .split("---")
            .filter(|s| !s.trim().is_empty())
            .map(|section_md| {
                let parser = pulldown_cmark::Parser::new(section_md.trim());
                let mut html_output = String::new();
                pulldown_cmark::html::push_html(&mut html_output, parser);
                (section_md.trim().to_string(), html_output)
            })
            .collect();

        let title = article.title.clone();

        // Krok 2: Renderowanie widoku
        let content_fragment = html! {
            div class="container mx-auto px-4 pb-16 lg:pb-24" {
                div class="max-w-4xl mx-auto" {
                    // Sekcja tytułowa (bez zmian)
                    div class="text-center mb-12" {
                        @if let Some(published_at) = article.published_at {
                            p class="text-sm text-slate-400 mb-4" { (published_at.format("%d %B %Y")) }
                        }
                        h1 class="text-4xl lg:text-6xl font-bold tracking-tighter bg-gradient-to-r from-brand-cyan to-brand-green text-transparent bg-clip-text" {
                            (title)
                        }
                    }

                    // Pętla renderująca - już bez bloku diagnostycznego
                    @for (_, rendered_html) in &sections {
                        // Przywracamy większy odstęp między sekcjami dla lepszej czytelności
                        section class="py-10" {
                            div class="prose prose-invert prose-xl" {
                                (PreEscaped(rendered_html))
                            }
                        }
                    }

                    // Przycisk powrotny (bez zmian)
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
