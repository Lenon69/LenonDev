// src/components/privacy.rs
use maud::{Markup, html};

pub fn privacy_policy_page() -> Markup {
    html! {
        div class="container mx-auto px-4 pb-16 lg:pb-24 pt-36 md:pt-28" {
            div class="max-w-4xl mx-auto" {
                // --- Nagłówek ---
                div class="text-center mb-12" {
                    h1 class="text-4xl lg:text-5xl font-bold tracking-tighter text-brand-cyan" { "Polityka Cookies" }
                    p class="mt-4 text-lg text-slate-400" { "Dbamy o Twoją prywatność. Poniżej znajdziesz szczegóły." }
                }

                // --- Kontener na wszystkie sekcje ---
                div class="space-y-8" {

                    // --- Sekcja 1: Czym są cookies ---
                    div class="bg-slate-800/50 p-6 lg:p-8 rounded-xl border border-slate-700/50" {
                        h2 class="text-2xl font-bold text-brand-green mb-4" { "1. Czym są pliki cookies?" }
                        p class="text-slate-300 leading-relaxed" { "Pliki cookies (tzw. „ciasteczka”) to niewielkie pliki tekstowe zapisywane na Twoim urządzeniu (komputerze, smartfonie) podczas przeglądania strony. Ułatwiają one korzystanie z witryny, np. zapamiętując preferencje i zbierając anonimowe dane statystyczne." }
                    }

                    // --- Sekcja 2: Jakich cookies używamy ---
                    div class="bg-slate-800/50 p-6 lg:p-8 rounded-xl border border-slate-700/50" {
                        h2 class="text-2xl font-bold text-brand-green mb-4" { "2. Jakich plików cookies używamy?" }
                        p class="text-slate-300 leading-relaxed mb-4" { "Nasza strona korzysta wyłącznie z Google Analytics, aby zbierać anonimowe dane statystyczne o sposobie korzystania z witryny w celu jej udoskonalania. W szczególności:" }
                        ul class="space-y-3" {
                            li class="flex items-start" {
                                (icon_check())
                                span class="ml-3 text-slate-300" { "Śledzimy liczbę odwiedzin, źródła ruchu i czas spędzony na stronie." }
                            }
                            li class="flex items-start" {
                                (icon_check())
                                span class="ml-3 text-slate-300" { "Wszystkie zbierane dane są w pełni anonimowe i nie pozwalają na Twoją identyfikację." }
                            }
                        }
                    }

                    // --- Sekcja 3: Zarządzanie ---
                    div class="bg-slate-800/50 p-6 lg:p-8 rounded-xl border border-slate-700/50" {
                        h2 class="text-2xl font-bold text-brand-green mb-4" { "3. Zarządzanie plikami cookies" }
                        p class="text-slate-300 leading-relaxed mb-4" { "Możesz w każdej chwili zmienić ustawienia dotyczące plików cookies w swojej przeglądarce. Pamiętaj jednak, że może to wpłynąć na działanie niektórych funkcji. Więcej informacji znajdziesz w dokumentacji:" }
                        div class="grid grid-cols-1 sm:grid-cols-3 gap-4" {
                            (browser_link("https://support.google.com/chrome", "Chrome"))
                            (browser_link("https://support.mozilla.org", "Firefox"))
                            (browser_link("https://support.microsoft.com", "Edge"))
                        }
                    }

                    // --- Sekcja 4: Google Analytics ---
                    div class="bg-slate-800/50 p-6 lg:p-8 rounded-xl border border-slate-700/50" {
                        h2 class="text-2xl font-bold text-brand-green mb-4" { "4. Narzędzie Google Analytics" }
                        p class="text-slate-300 leading-relaxed mb-4" { "Dane zbierane przez Google Analytics są przetwarzane zgodnie z polityką prywatności Google. Możesz zablokować działanie Google Analytics, instalując oficjalną wtyczkę do przeglądarki:" }
                        a href="https://tools.google.com/dlpage/gaoptout" target="_blank" rel="noopener noreferrer"
                           class="inline-block bg-brand-purple hover:opacity-80 transition-opacity text-white font-bold py-2 px-6 rounded-lg" {
                            "Zablokuj Google Analytics"
                        }
                    }

                    // --- Sekcja 5: Zmiany i kontakt ---
                    div class="bg-slate-800/50 p-6 lg:p-8 rounded-xl border border-slate-700/50" {
                        h2 class="text-2xl font-bold text-brand-green mb-4" { "5. Zmiany i kontakt" }
                        p class="text-slate-300 leading-relaxed" { "Zastrzegamy sobie prawo do wprowadzania zmian w niniejszej polityce. Jeśli masz pytania, skontaktuj się z nami przez formularz kontaktowy." }
                    }
                }
            }
        }
    }
}

// Funkcja pomocnicza do generowania ikony "check"
fn icon_check() -> Markup {
    html! {
        svg class="flex-shrink-0 h-6 w-6 text-brand-cyan" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2" {
            path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7" {}
        }
    }
}

// Funkcja pomocnicza do generowania linków do przeglądarek
fn browser_link(href: &str, name: &str) -> Markup {
    html! {
        a href=(href) target="_blank" rel="noopener noreferrer"
           class="block text-center bg-slate-700/50 hover:bg-slate-600/50 p-3 rounded-lg text-slate-300 font-semibold transition-colors" {
            (name)
        }
    }
}
