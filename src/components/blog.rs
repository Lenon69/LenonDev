// src/components/blog.rs
use crate::models::Article;
use maud::{Markup, html};

// Komponent dla strony /blog (lista artykułów)
pub fn blog_index_view(articles: Vec<Article>, current_page: i64, total_pages: i64) -> Markup {
    html! {
        div id="blog-list-container" class="bg-[#1A1A1E]/50 pb-20 lg:pb-24 pt-36 md:pt-28" hx-target="this" hx-swap="outerHTML" {
            div class="container mx-auto px-4" {
                div class="text-center mb-16" {
                    h1 class="text-4xl lg:text-5xl font-bold tracking-tight text-brand-cyan" {"Wpisy na blogu"}
                    p class="mt-4 text-lg text-slate-400" {"Dzielę się tutaj swoją wiedzą i przemyśleniami."}
                }

                div class="max-w-4xl mx-auto space-y-12" {
                    @if articles.is_empty() {
                        p class="text-center text-slate-400" { "Brak wpisów na tej stronie." }
                    }
                    @for article in articles {
                        // Ten link używa HTMX, aby dynamicznie załadować treść artykułu
                        a href=(format!("/blog/{}", article.slug))
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

                // --- NOWA SEKCJA: NAWIGACJA PAGINACJI ---
                @if total_pages > 1 {
                    div class="flex justify-between items-center max-w-4xl mx-auto mt-16" {
                        // Przycisk "Poprzednia" (widoczny, jeśli nie jesteśmy na pierwszej stronie)
                        @if current_page > 1 {
                            a
                            href=(format!("/blog?page={}", current_page - 1))
                            hx-get=(format!("/blog?page={}", current_page - 1)) class="cursor-pointer inline-block bg-slate-700 hover:bg-slate-600 transition-colors text-white font-bold py-2 px-6 rounded-lg" {
                                "← Nowsze wpisy"
                            }
                        } @else {
                            // Pusty div, aby utrzymać symetrię layoutu
                            div class="py-2 px-6" {}
                            }

                        // Informacja o bieżącej stronie
                        span class="text-slate-400" {
                            "Strona " (current_page) " z " (total_pages)
                        }

                        // Przycisk "Następna" (widoczny, jeśli nie jesteśmy na ostatniej stronie)
                        @if current_page < total_pages {
                            a
                            href=(format!("/blog?page={}", current_page + 1))
                            hx-get=(format!("/blog?page={}", current_page + 1)) class="cursor-pointer inline-block bg-slate-700 hover:bg-slate-600 transition-colors text-white font-bold py-2 px-6 rounded-lg" {
                                "Starsze wpisy →"
                            }
                        } @else {
                            div {}
                        }
                    }
                }
            }
        }
    }
}
