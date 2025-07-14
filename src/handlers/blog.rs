// src/handlers/blog.rs
use crate::{AppState, components::blog, models::Article};
use axum::{
    extract::{Path, State},
    response::Html,
};
use maud::{Markup, PreEscaped, html};

// Handler dla /blog (pozostaje bez zmian)
pub async fn blog_index(State(state): State<AppState>) -> Html<Markup> {
    let articles = sqlx::query_as::<_, Article>(
        "SELECT * FROM articles WHERE published_at IS NOT NULL ORDER BY published_at DESC",
    )
    .fetch_all(&state.db_pool)
    .await
    .unwrap_or_else(|_| vec![]);
    Html(blog::blog_index_view(articles))
}

pub async fn show_article(State(state): State<AppState>, Path(slug): Path<String>) -> Html<Markup> {
    if let Ok(article) = sqlx::query_as::<_, Article>(
        "SELECT * FROM articles WHERE slug = $1 AND published_at IS NOT NULL",
    )
    .bind(slug)
    .fetch_one(&state.db_pool)
    .await
    {
        // === KROK 1: PRZYGOTOWANIE TREŚCI ===
        // Dzielimy treść na sekcje i od razu konwertujemy każdą na HTML,
        // zbierając wyniki w nowym wektorze `html_sections`.
        let html_sections: Vec<String> = article
            .content
            .split("---")
            .filter(|s| !s.trim().is_empty()) // Pomijamy puste sekcje
            .map(|section_md| {
                let parser = pulldown_cmark::Parser::new(section_md);
                let mut html_output = String::new();
                pulldown_cmark::html::push_html(&mut html_output, parser);
                html_output
            })
            .collect();

        // === KROK 2: RENDEROWANIE PRZYGOTOWANYCH DANYCH ===
        // Teraz makro `html!` ma proste zadanie: iterować po gotowych fragmentach HTML.
        Html(html! {
            div class="container mx-auto px-4 py-16 lg:py-24" {
                div class="max-w-4xl mx-auto" {

                    // Sekcja tytułowa
                    div class="text-center mb-12" {
                        @if let Some(published_at) = article.published_at {
                            p class="text-sm text-slate-400 mb-4" { (published_at.format("%d %B %Y")) }
                        }
                        h1 class="text-4xl lg:text-6xl font-bold tracking-tighter bg-gradient-to-r from-brand-cyan to-brand-green text-transparent bg-clip-text" {
                            (article.title)
                        }
                    }

                    // Renderowanie każdej sekcji
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
                                      prose-li:before:bg-[url('/svg/checkmark.svg')] prose-li:before:bg-contain"
                            {
                                (PreEscaped(section_html))
                            }
                        }
                    }

                    // Przycisk powrotny
                    div class="text-center mt-16" {
                        a hx-get="/blog" hx-target="#content-area" hx-push-url="/blog"
                          class="cursor-pointer inline-block bg-slate-700 hover:bg-slate-600 transition-colors text-white font-bold py-2 px-6 rounded-lg" {
                            "← Wróć na bloga"
                        }
                    }
                }
            }
        })
    } else {
        // Komunikat błędu
        Html(html! {
            div class="text-center py-40" {
                h1 class="text-2xl text-red-500" { "404 - Nie znaleziono artykułu" }
            }
        })
    }
}
