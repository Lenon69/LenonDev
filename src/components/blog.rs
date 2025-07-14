// src/components/blog.rs
use crate::models::Article;
use maud::{Markup, PreEscaped, html};

// Komponent dla strony /blog (lista artykułów)
pub fn blog_index_view(articles: Vec<Article>) -> Markup {
    html! {
        div class="bg-[#1A1A1E]/50 py-20 lg:py-24" {
            div class="container mx-auto px-4" {
                div class="text-center mb-16" {
                    h1 class="text-4xl lg:text-5xl font-bold tracking-tight text-brand-cyan" {"Wpisy na blogu"}
                    p class="mt-4 text-lg text-slate-400" {"Dzielę się tutaj swoją wiedzą i przemyśleniami."}
                }

                div class="max-w-4xl mx-auto space-y-12" {
                    @for article in articles {
                        a href=(format!("/blog/{}", article.slug)) class="block group" {
                            article class="bg-slate-800/40 hover:bg-slate-800/80 p-6 rounded-lg border border-slate-700/50 transition-colors duration-300" {
                                // Data publikacji
                                @if let Some(published_at) = article.published_at {
                                    p class="text-sm text-slate-400 mb-2" { (published_at.format("%d %B %Y")) }
                                }
                                // Tytuł
                                h2 class="text-2xl font-bold text-brand-green group-hover:text-brand-cyan transition-colors duration-300" { (article.title) }
                                // Zajawka
                                @if let Some(excerpt) = &article.excerpt {
                                    p class="text-slate-300 mt-3" { (excerpt) }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// Komponent dla strony /blog/{slug} (pojedynczy artykuł)
pub fn article_detail_view(article: Article, content_html: String) -> Markup {
    html! {
        div class="bg-brand-dark py-20 lg:py-24" {
            // Używamy klas `prose` od Tailwind Typography do stylowania treści
            article class="prose prose-invert prose-lg lg:prose-xl mx-auto px-4" {
                // Tytuł
                h1 { (article.title) }
                // Data
                @if let Some(published_at) = article.published_at {
                    p class="text-base text-slate-400 !mt-2" { (format!("Opublikowano: {}", published_at.format("%d %B %Y"))) }
                }

                // Właściwa treść artykułu (przetworzona z Markdown)
                div class="mt-8 border-t border-slate-700/50 pt-8" {
                    (PreEscaped(content_html))
                }
            }
            // Link powrotny
            div class="text-center mt-16" {
                a href="/blog" class="inline-block bg-slate-700 hover:bg-slate-600 transition-colors text-white font-bold py-2 px-6 rounded-lg" {
                    "← Wróć na bloga"
                }
            }
        }
    }
}
