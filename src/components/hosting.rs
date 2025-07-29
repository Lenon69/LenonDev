// src/components/hosting.rs
use maud::{Markup, html};

use crate::components::cta::whatsapp_button;

pub fn hosting_page_view() -> Markup {
    html! {
        div class="container mx-auto px-4 pb-16 lg:pb-24 pt-36 md:pt-28 animate-fade-in" {
            // Nagłówek
            div class="text-center mb-16" {
                h1 class="text-4xl lg:text-6xl font-bold tracking-tighter bg-gradient-to-r from-brand-cyan to-brand-green text-transparent bg-clip-text py-4" {
                    "Hosting i Domena"
                }
                p class="mt-4 text-lg text-slate-400 max-w-3xl mx-auto" {
                    "Zapewnij swojej stronie solidny fundament. Pomogę Ci wybrać, skonfigurować i zarządzać hostingiem oraz domeną."
                }
            }

            // Główne sekcje
            div class="max-w-4xl mx-auto space-y-12" {
                (feature_section(
                    "Dobór Hostingu",
                    "Wybór odpowiedniego hostingu to klucz do szybkości i niezawodności. Doradzę Ci najlepsze rozwiązanie (VPS, chmura) dopasowane do Twojego projektu i budżetu, abyś nie przepłacał za niewykorzystane zasoby."
                ))
                (feature_section(
                    "Konfiguracja i Migracja",
                    "Zajmę się pełną konfiguracją serwera, certyfikatów SSL i poczty e-mail. Jeśli posiadasz już stronę, pomogę w jej bezpiecznej migracji do nowego, lepszego środowiska."
                ))
                (feature_section(
                    "Rejestracja Domeny",
                    "Pomogę Ci znaleźć i zarejestrować idealną domenę dla Twojej marki, a następnie poprawnie ją skonfiguruję, aby wskazywała na Twoją nową stronę."
                ))
            }

            (whatsapp_button("Usługa - Hosting"))
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
