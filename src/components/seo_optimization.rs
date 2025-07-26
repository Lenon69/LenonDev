// src/components/seo_optimization.rs
use maud::{Markup, html};

pub fn seo_optimization_page_view() -> Markup {
    html! {
        div class="container mx-auto px-4 pb-16 lg:pb-24 pt-36 md:pt-28 animate-fade-in" {
            // Nagłówek
            div class="text-center mb-16" {
                h1 class="text-4xl lg:text-6xl font-bold tracking-tighter bg-gradient-to-r from-brand-cyan to-brand-green text-transparent bg-clip-text py-4" {
                    "Optymalizacja i SEO"
                }
                p class="mt-4 text-lg text-slate-400 max-w-3xl mx-auto" {
                    "Zwiększ widoczność swojej strony w internecie i dotrzyj do nowych klientów dzięki profesjonalnej optymalizacji."
                }
            }

            // Główne sekcje
            div class="max-w-4xl mx-auto space-y-12" {
                (feature_section(
                    "Audyt i Analiza",
                    "Zaczynamy od dogłębnej analizy Twojej strony pod kątem technicznym i treści. Identyfikujemy kluczowe obszary do poprawy, analizujemy konkurencję i dobieramy najlepsze słowa kluczowe dla Twojej branży."
                ))
                (feature_section(
                    "Optymalizacja On-Page",
                    "Dostosowujemy treści, meta tagi, nagłówki i strukturę linków wewnętrznych, aby Twoja strona była w pełni zrozumiała dla wyszukiwarek i atrakcyjna dla użytkowników."
                ))
                (feature_section(
                    "Optymalizacja Techniczna",
                    "Skupiamy się na prędkości ładowania, responsywności i danych strukturalnych (Schema.org). Zapewniamy, że Twoja strona spełnia wszystkie najnowsze wytyczne techniczne Google."
                ))
            }

            // Sekcja powrotu
            div class="text-center mt-20" {
                a href="/oferta" hx-get="/oferta" hx-target="#content-area" hx-push-url="/oferta" class="cursor-pointer inline-block bg-slate-700 hover:bg-slate-600 transition-colors text-white font-bold py-3 px-8 rounded-lg" {
                    "← Wróć do Głównej Oferty"
                }
            }
        }
    }
}

// Funkcja pomocnicza dla sekcji
fn feature_section(title: &str, description: &str) -> Markup {
    html! {
        div class="bg-slate-800/50 p-8 rounded-xl border border-slate-700/50" {
            h2 class="text-2xl font-bold text-brand-cyan mb-4" { (title) }
            p class="text-slate-300 leading-relaxed" { (description) }
        }
    }
}
