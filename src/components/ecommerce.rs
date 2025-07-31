// src/components/ecommerce.rs
use crate::components::{
    cta::whatsapp_button,
    simple_site::{back_to_offer_button, feature_card},
};
use maud::{Markup, html};

pub fn ecommerce_page_view() -> Markup {
    html! {
        div class="container mx-auto px-4 pb-16 lg:pb-24 pt-36 md:pt-28 animate-fade-in" {
            div class="text-center mb-16" {
                span class="bg-brand-purple/10 text-brand-purple border border-brand-purple/30 rounded-full px-4 py-1 font-medium" { "E-commerce" }
                h1 class="text-4xl lg:text-6xl font-bold tracking-tighter mt-4" { "Sprzedawaj Skutecznie w Sieci" }
                p class="mt-4 text-lg text-slate-400 max-w-3xl mx-auto" {
                    "Stworzę dla Ciebie kompletne i wydajne rozwiązanie e-commerce, które pozwoli Ci zarządzać produktami, przyjmować płatności i rozwijać swój biznes online."
                }
            }

            div class="max-w-4xl mx-auto" {
                h2 class="text-3xl font-bold text-center text-brand-green mb-10" { "Kluczowe funkcje sklepu" }
                div class="grid grid-cols-1 md:grid-cols-2 gap-8" {
                    (feature_card(
                        "Zarządzanie Produktami",
                        "Prosty panel do dodawania produktów, kategorii, wariantów i zarządzania stanem magazynowym.",
                        "📦"
                    ))
                    (feature_card(
                        "Integracja Płatności",
                        "Bezpieczna integracja z popularnymi bramkami płatności (np. Stripe, Przelewy24), aby Twoi klienci mogli płacić szybko i wygodnie.",
                        "💳"
                    ))
                    (feature_card(
                        "Zarządzanie Zamówieniami",
                        "System do śledzenia i obsługi zamówień, od złożenia po wysyłkę.",
                        "🚚"
                    ))
                     (feature_card(
                        "Niezawodność i Skalowalność",
                        "Sklep zbudowany na solidnych fundamentach technologii Rust jest gotowy na obsługę dużego ruchu i dalszy rozwój Twojego biznesu.",
                        "📈"
                    ))
                }
            }

            (whatsapp_button("Sklep internetowy"))
            (back_to_offer_button())
        }
    }
}
