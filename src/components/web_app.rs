// src/components/web_app.rs
use crate::components::simple_site::{back_to_offer_button, feature_card};
use maud::{Markup, html};

pub fn web_app_page_view() -> Markup {
    html! {
        div class="container mx-auto px-4 pb-16 lg:pb-24 pt-36 md:pt-28 animate-fade-in" {
            div class="text-center mb-16" {
                span class="bg-brand-purple/10 text-brand-purple border border-brand-purple/30 rounded-full px-4 py-1 font-medium" { "Aplikacja Webowa" }
                h1 class="text-4xl lg:text-6xl font-bold tracking-tighter mt-4" { "Zautomatyzuj Swój Biznes" }
                p class="mt-4 text-lg text-slate-400 max-w-3xl mx-auto" {
                    "Tworzę zaawansowane aplikacje webowe, takie jak systemy CRM, platformy rezerwacyjne czy wewnętrzne narzędzia, które usprawniają i automatyzują procesy w Twojej firmie."
                }
            }

            div class="max-w-4xl mx-auto" {
                h2 class="text-3xl font-bold text-center text-brand-green mb-10" { "Przykładowe Zastosowania" }
                div class="grid grid-cols-1 md:grid-cols-2 gap-8" {
                    (feature_card(
                        "Systemy CRM",
                        "Zarządzaj relacjami z klientami, śledź sprzedaż i automatyzuj komunikację w jednym, dedykowanym narzędziu.",
                        "👥"
                    ))
                    (feature_card(
                        "Platformy Rezerwacyjne",
                        "System do rezerwacji wizyt, terminów czy zasobów, zintegrowany z kalendarzem i płatnościami online.",
                        "🗓️"
                    ))
                    (feature_card(
                        "Narzędzia Analityczne",
                        "Dedykowane dashboardy i panele do wizualizacji danych, które pomogą Ci podejmować lepsze decyzje biznesowe.",
                        "📈"
                    ))
                     (feature_card(
                        "Wewnętrzne Systemy Firmowe",
                        "Aplikacje do zarządzania projektami, dokumentami, zasobami ludzkimi – wszystko w jednym, bezpiecznym miejscu.",
                        "🏢"
                    ))
                }
            }

            (back_to_offer_button())
        }
    }
}
