// src/components/blog.rs
use crate::models::Article;
use maud::{Markup, html};

// Komponent dla strony /blog (lista artykułów)
pub fn blog_index_view(articles: Vec<Article>) -> Markup {
    html! {
        div class="bg-[#1A1A1E]/50 pb-20 lg:pb-24 pt-36 md:pt-28" {
            div class="container mx-auto px-4" {
                div class="text-center mb-16" {
                    h1 class="text-4xl lg:text-5xl font-bold tracking-tight text-brand-cyan" {"Wpisy na blogu"}
                    p class="mt-4 text-lg text-slate-400" {"Dzielę się tutaj swoją wiedzą i przemyśleniami."}
                }

                div class="max-w-4xl mx-auto space-y-12" {
                    @for article in articles {
                        // Ten link używa HTMX, aby dynamicznie załadować treść artykułu
                        a hx-get=(format!("/blog/{}", article.slug)) hx-target="#content-area" hx-push-url=(format!("/blog/{}", article.slug))
                          class="block group cursor-pointer" {
                            article class="bg-slate-800/40 hover:bg-slate-800/80 p-6 rounded-lg border border-slate-700/50 transition-all duration-300 hover:shadow-cyan-glow hover:-translate-y-1" {
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
