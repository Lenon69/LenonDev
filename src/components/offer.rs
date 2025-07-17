// src/components/offer.rs
use maud::{Markup, html};

// Główny komponent widoku dla podstrony /oferta
pub fn offer_page_view() -> Markup {
    html! {
        // Główny kontener z odpowiednim paddingiem
        div class="container mx-auto px-4 pb-16 lg:pb-24" {

            // --- SEKCJA NAGŁÓWKOWA ---
            div class="text-center mb-16 md:mb-20" {
                h1 class="text-4xl lg:text-6xl font-bold tracking-tighter bg-gradient-to-r from-brand-cyan to-brand-green text-transparent bg-clip-text" {
                    "Oferta i Usługi"
                }
                p class="mt-4 text-lg text-slate-400 max-w-2xl mx-auto" {
                    "Nowoczesne rozwiązania webowe, które pomogą Twojej firmie zaistnieć w internecie i osiągnąć sukces."
                }
            }

            // --- SEKCJA: TWORZENIE STRON INTERNETOWYCH ---
            div {
                h2 class="text-3xl font-bold text-center text-slate-100 mb-10" { "Tworzenie Stron Internetowych" }
                // Grid z kartami dla każdego typu strony
                div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8" {
                    // Karta: Strona Wizytówka
                    (offer_card(
                        "Prosta Strona Wizytówka",
                        "Idealna na start – profesjonalnie zaprezentuj swoją firmę, usługi i dane kontaktowe.",
                        "1 000 - 3 000 zł"
                    ))
                    // Karta: Landing Page
                    (offer_card(
                        "Landing Page",
                        "Skupiona na jednym celu – idealna do kampanii marketingowych, promocji produktu lub zapisu na newsletter.",
                        "1 700 - 3 500 zł"
                    ))
                    // Karta: Strona z Blogiem
                    (offer_card(
                        "Strona z Blogiem / CMS",
                        "Dziel się wiedzą i buduj pozycję eksperta. Prosty w obsłudze system do zarządzania treścią.",
                        "2 500 - 5 000 zł"
                    ))
                    // Karta: Sklep Internetowy
                    (offer_card(
                        "Sklep Internetowy",
                        "Sprzedawaj swoje produkty online. Kompletne rozwiązanie e-commerce z płatnościami i zarządzaniem.",
                        "7 000 - 25 000 zł"
                    ))
                    // Karta: Projekt Indywidualny
                    (offer_card(
                        "Projekt Indywidualny",
                        "Masz unikalny pomysł? Stworzę dedykowaną aplikację webową idealnie dopasowaną do Twoich potrzeb.",
                        "od 5 000 zł"
                    ))
                }
            }

            // --- SEKCJA: USŁUGI DODATKOWE ---
            div class="mt-20" {
                h2 class="text-3xl font-bold text-center text-slate-100 mb-10" { "Usługi Dodatkowe" }
                div class="grid grid-cols-1 md:grid-cols-3 gap-8 text-center" {
                    // Karta: Opieka nad stroną
                    (service_card(
                        html!{ "Opieka nad Stroną" },
                        "Aktualizacje, kopie zapasowe i monitoring bezpieczeństwa. Śpij spokojnie, ja czuwam nad Twoją stroną."
                    ))
                    // Karta: Optymalizacja
                    (service_card(
                        html!{ "Optymalizacja i SEO" },
                        "Poprawa szybkości ładowania strony i optymalizacja pod kątem wyszukiwarek (SEO), aby klienci mogli Cię znaleźć."
                    ))
                    // Karta: Hosting i Domena
                    (service_card(
                        html!{ "Hosting i Domena" },
                        "Pomoc w wyborze i konfiguracji szybkiego, niezawodnego hostingu oraz rejestracji idealnej domeny."
                    ))
                }
            }
        }
    }
}

// Komponent pomocniczy do tworzenia kart z ofertą
fn offer_card(title: &str, description: &str, price: &str) -> Markup {
    html! {
        div class="bg-slate-800/50 p-6 rounded-xl border border-slate-700/50 transition-all duration-300 hover:border-brand-cyan/50 hover:shadow-cyan-glow hover:-translate-y-1 flex flex-col" {
            h3 class="text-xl font-bold text-brand-cyan mb-3" { (title) }
            p class="text-slate-400 flex-grow" { (description) }
            div class="mt-6 pt-4 border-t border-slate-700" {
                p class="text-lg font-bold text-slate-100 text-center" { (price) }
            }
        }
    }
}

// Komponent pomocniczy do tworzenia kart z usługami dodatkowymi
fn service_card(title: Markup, description: &str) -> Markup {
    html! {
        div class="bg-slate-800/50 p-6 rounded-xl border border-slate-700/50" {
            h3 class="text-xl font-bold text-slate-200 mb-3" { (title) }
            p class="text-slate-400" { (description) }
        }
    }
}
