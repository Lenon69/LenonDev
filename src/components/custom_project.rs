// src/components/custom_project.rs
use crate::components::simple_site::{back_to_offer_button, feature_card};
use maud::{Markup, html};

pub fn custom_project_page_view() -> Markup {
    html! {
        div class="container mx-auto px-4 pb-16 lg:pb-24 pt-36 md:pt-28 animate-fade-in" {
            div class="text-center mb-16" {
                span class="bg-brand-purple/10 text-brand-purple border border-brand-purple/30 rounded-full px-4 py-1 font-medium" { "Projekt Indywidualny" }
                h1 class="text-4xl lg:text-6xl font-bold tracking-tighter mt-4" { "Zamień Pomysł w Rzeczywistość" }
                p class="mt-4 text-lg text-slate-400 max-w-3xl mx-auto" {
                    "Masz unikalny pomysł na aplikację, platformę lub narzędzie online? Razem możemy stworzyć rozwiązanie idealnie dopasowane do Twoich potrzeb."
                }
            }

            div class="max-w-4xl mx-auto" {
                h2 class="text-3xl font-bold text-center text-brand-green mb-10" { "Jak Wygląda Proces?" }
                div class="grid grid-cols-1 md:grid-cols-2 gap-8" {
                    (feature_card(
                        "Konsultacja i Planowanie",
                        "Zaczynamy od szczegółowej rozmowy o Twoim pomyśle. Definiujemy cele, funkcje i plan działania, aby mieć pewność, że idziemy w dobrym kierunku.",
                        "palette"
                    ))
                    (feature_card(
                        "Prototypowanie i Design",
                        "Tworzymy interaktywne makiety (UX/UI), abyś mógł zobaczyć i przetestować wygląd oraz działanie aplikacji jeszcze przed napisaniem kodu.",
                        "rocket"
                    ))
                    (feature_card(
                        "Development i Testy",
                        "Budujemy Twoją aplikację, regularnie informując Cię o postępach. Każdy element jest dokładnie testowany pod kątem wydajności i bezpieczeństwa.",
                        "trending-up"
                    ))
                     (feature_card(
                        "Wdrożenie i Wsparcie",
                        "Pomagam we wdrożeniu gotowego produktu na serwer i oferuję wsparcie techniczne po zakończeniu projektu, aby zapewnić jego płynne działanie.",
                        "device-mobile"
                    ))
                }
            }

            (back_to_offer_button())
        }
    }
}
