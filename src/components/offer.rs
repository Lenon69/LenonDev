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
                h2 class="text-3xl font-bold text-center text-brand-green mb-10" { "Tworzenie Stron Internetowych" }
                // Grid z kartami dla każdego typu strony
                div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8" {
                    // Karta: Strona Wizytówka
                    (offer_card(
                        "Prosta Strona Wizytówka",
                        "Idealna na start – profesjonalnie zaprezentuj swoją firmę, usługi i dane kontaktowe.",
                        "1 500 - 4 000 zł"
                    ))
                    // Karta: Landing Page
                    (offer_card(
                        "Landing Page",
                        "Skupiona na jednym celu – idealna do kampanii marketingowych, promocji produktu lub zapisu na newsletter.",
                        "2 000 - 4 000 zł"
                    ))
                    // Karta: Strona z Blogiem
                    (offer_card(
                        "Strona z Blogiem / CMS",
                        "Dziel się wiedzą i buduj pozycję eksperta. Prosty w obsłudze system do zarządzania treścią.",
                        "5000 - 9 000 zł"
                    ))
                    // Karta: Sklep Internetowy
                    (offer_card(
                        "Sklep Internetowy",
                        "Sprzedawaj swoje produkty online. Kompletne rozwiązanie e-commerce z płatnościami i zarządzaniem.",
                        "8 000 - 22 000 zł"
                    ))
                    // Karta: Projekt Indywidualny
                    (offer_card(
                        "Projekt Indywidualny",
                        "Masz unikalny pomysł? Stworzę dedykowaną aplikację webową idealnie dopasowaną do Twoich potrzeb.",
                        "od 5 000 zł"
                    ))
                    // Karta: Aplikacja Webowa
                    (offer_card(
                        "Aplikacja webowa (CRM)",
                        "Zaawansowana logika backendowa.",
                        "od 10 000 zł - 50 0000+ zł"
                    ))
                }
            }

            // --- SEKCJA: USŁUGI DODATKOWE ---
            div class="mt-20" {
                h2 class="text-3xl font-bold text-center text-brand-green mb-10" { "Usługi Dodatkowe" }
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
            // --- SEKCJA: DLACZEGO WARTO? ---
            div class="mt-20" {
                h2 class="text-3xl font-bold text-center text-brand-green mb-10" { "Co Otrzymujesz w Cenie?" }
                div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8" {
                    // Karta: Bezpieczeństwo i Szybkość
                    (feature_card(
                        html! {
                            svg xmlns="http://www.w3.org/2000/svg" class="h-8 w-8 text-brand-cyan" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2" {
                                path stroke-linecap="round" stroke-linejoin="round" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 20.944a12.02 12.02 0 009 3c4.595 0 8.525-2.686 10.23-6.581a12.025 12.025 0 00-4.612-11.528z" {}
                            }
                        },
                        "Bezpieczeństwo i Szybkość",
                        "Twoja strona powstaje w języku Rust, technologii znanej z niezrównanego bezpieczeństwa i wydajności. To nie szablon z WordPressa, a solidny, autorski kod."
                    ))

                    // Karta: Lekkość i Oszczędność
                    (feature_card(
                        html! {
                            svg xmlns="http://www.w3.org/2000/svg" class="h-8 w-8 text-brand-cyan" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2" {
                                path stroke-linecap="round" stroke-linejoin="round" d="M5 3v4M3 5h4M6 17v4m-2-2h4m5-16l2.286 6.857L21 12l-5.714 2.143L13 21l-2.286-6.857L5 12l5.714-2.143L13 3z" {}
                            }
                        },
                        "Lekkość i Oszczędność",
                        "Dzięki optymalizacji, tworzone przeze mnie strony są niezwykle lekkie i nie wymagają drogich serwerów. Oszczędzasz na hostingu, nie tracąc na prędkości."
                    ))

                    // Karta: Nowoczesny Frontend
                    (feature_card(
                        html! {
                            svg xmlns="http://www.w3.org/2000/svg" class="h-8 w-8 text-brand-cyan" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2" {
                                path stroke-linecap="round" stroke-linejoin="round" d="M14 10l-2 1m0 0l-2-1m2 1v2.5M20 7l-2 1m2-1l-2-1m2 1v2.5M14 4l-2-1-2 1M4 7l2 1M4 7l2-1M4 7v2.5M12 21l-2-1m2 1l2-1m-2 1v-2.5M6 18l-2-1v-2.5M18 18l2-1v-2.5" {}
                            }
                        },
                        "Nowoczesny Frontend",
                        "Zamiast ciężkich frameworków jak React, stosuję lekki stos HTMX + Alpine.js. Efekt? Błyskawiczny interfejs i doskonałe doświadczenie użytkownika (UX)."
                    ))

                    // Karta: Rozwiązania "Szyte na Miarę"
                    (feature_card(
                        html! {
                            svg xmlns="http://www.w3.org/2000/svg" class="h-8 w-8 text-brand-cyan" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2" {
                                path stroke-linecap="round" stroke-linejoin="round" d="M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" {}
                            }
                        },
                        "Rozwiązania \"Szyte na Miarę\"",
                        "Otrzymujesz system zaprojektowany od zera, dokładnie pod Twoje potrzeby. Płacisz tylko za te funkcje, których naprawdę potrzebujesz, bez zbędnego balastu."
                    ))

                    // Karta: Kompleksowa Opieka
                    (feature_card(
                        html! {
                            svg xmlns="http://www.w3.org/2000/svg" class="h-8 w-8 text-brand-cyan" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2" {
                                path stroke-linecap="round" stroke-linejoin="round" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" {}
                            }
                        },
                        "Kompleksowa Opieka",
                        "Moja usługa to nie tylko kod. To także dbałość o techniczne SEO, topową wydajność, bezpieczeństwo i przyszły rozwój Twojej strony."
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
        div class="bg-slate-800/50 p-6 rounded-xl border border-slate-700/50 transition-all duration-300 hover:border-brand-cyan/50 hover:shadow-cyan-glow hover:-translate-y-1" {
            h3 class="text-xl font-bold text-slate-200 mb-3" { (title) }
            p class="text-slate-400" { (description) }
        }
    }
}

fn feature_card(icon: Markup, title: &str, description: &str) -> Markup {
    html! {
        div class="bg-slate-800/50 p-6 rounded-xl border border-slate-700/50 text-center transition-all duration-300 hover:border-brand-cyan/50 hover:shadow-cyan-glow hover:-translate-y-1" {
            div class="flex justify-center mb-4" {
                (icon)
            }
            h3 class="text-xl font-bold text-slate-100 mb-2" { (title) }
            p class="text-slate-400" { (description) }
        }
    }
}
