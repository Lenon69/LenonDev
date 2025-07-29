// src/components/simple_site.rs
use maud::{Markup, html};

use crate::components::cta::whatsapp_button;

pub fn simple_site_page_view() -> Markup {
    html! {
        div class="container mx-auto px-4 pb-16 lg:pb-24 pt-36 md:pt-28 animate-fade-in" {
            // Nagłówek
            div class="text-center mb-16" {
                span class="bg-brand-purple/10 text-brand-purple border border-brand-purple/30 rounded-full px-4 py-1 font-medium" { "Strona Wizytówka" }
                h1 class="text-4xl lg:text-6xl font-bold tracking-tighter mt-4" { "Twoje Cyfrowe Oblicze" }
                p class="mt-4 text-lg text-slate-400 max-w-3xl mx-auto" {
                    "Profesjonalna strona-wizytówka to fundament obecności w internecie. To miejsce, gdzie klienci mogą Cię znaleźć, poznać Twoją ofertę i skontaktować się z Tobą."
                }
            }

            // Sekcja "Co Otrzymujesz?"
            div class="max-w-4xl mx-auto" {
                h2 class="text-3xl font-bold text-center text-brand-green mb-10" { "Co Otrzymujesz w Pakiecie?" }
                div class="grid grid-cols-1 md:grid-cols-2 gap-8" {
                    (feature_card(
                        "Indywidualny Projekt Graficzny",
                        "Zapomnij o szablonach. Twoja strona będzie unikalna, zaprojektowana od podstaw, aby odzwierciedlać charakter Twojej marki.",
                        "🎨"
                    ))
                    (feature_card(
                        "Ultra-szybkość",
                        "Strona oparta na technologii Rust + HTMX działa błyskawicznie, co przekłada się na lepsze doświadczenie użytkownika i wyższe pozycje w Google.",
                        "⚡️"
                    ))
                    (feature_card(
                        "Pełna Responsywność (RWD)",
                        "Idealne wyświetlanie na każdym urządzeniu – od smartfonów po duże monitory.",
                        "📱💻"
                    ))
                    (feature_card(
                        "Podstawowa Optymalizacja SEO",
                        "Przygotowujemy Twoją stronę pod kątem wyszukiwarek, aby klienci mogli Cię łatwiej znaleźć.",
                        "📈"
                    ))
                }
            }

            // Sekcja zakupu
            (whatsapp_button("Prosta Strona Wizytówka"))
            // Sekcja powrotu
            (back_to_offer_button())
        }
    }
}

// Funkcje pomocnicze
pub fn feature_card(title: &str, description: &str, emoji: &str) -> Markup {
    html! {
        div class="bg-slate-800/50 p-6 rounded-xl border border-slate-700/50 text-center" {
            div class="text-4xl mb-4" {
                (emoji)
            }
            h3 class="text-xl font-bold text-slate-100 mb-2" { (title) }
            p class="text-slate-400" { (description) }
        }
    }
}

pub fn back_to_offer_button() -> Markup {
    html! {
        div class="text-center mt-4" {
            a href="/oferta" hx-get="/oferta" hx-target="#content-area" hx-push-url="/oferta" class="cursor-pointer inline-block bg-slate-700 hover:bg-slate-600 transition-colors text-white font-bold py-3 px-8 rounded-lg" {
                "← Wróć do Głównej Oferty"
            }
        }
    }
}
