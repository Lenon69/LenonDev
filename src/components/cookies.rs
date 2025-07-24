// src/components/cookies.rs
use maud::{Markup, html};

pub fn cookies_banner() -> Markup {
    html! {
        // Ten div będzie kontrolowany przez Alpine.js
        // x-data: Inicjalizuje stan. Domyślnie banner jest ukryty (show: false)
        // x-init: Sprawdza, czy w Local Storage jest wpis 'cookiesAccepted'. Jeśli nie ma, pokazuje banner.
        // x-show: Pokazuje lub ukrywa element w zależności od wartości 'show'
        // x-cloak: Zapobiega "mignięciu" banera zanim Alpine.js go ukryje
        div
            x-cloak
            x-data="{ show: false }"
            x-init="!localStorage.getItem('cookiesAccepted') && (show = true)"
            x-show="show"
            class="fixed bottom-0 left-0 right-0 z-50 p-4"
        {
            // Właściwy wygląd banera
            div class="container mx-auto max-w-4xl bg-slate-800/90 backdrop-blur-sm border border-slate-700/50 rounded-lg shadow-2xl p-6 flex flex-col md:flex-row items-center justify-between gap-6" {
                // Tekst
                p class="text-slate-300 text-sm text-center md:text-left" {
                    "Ta strona wykorzystuje pliki cookies w celu poprawy doświadczenia użytkownika oraz do analizy ruchu. Korzystając ze strony, wyrażasz zgodę na ich użycie."
                }
                // Przycisk akceptacji
                button
                    // @click: Po kliknięciu, ustawia wpis w Local Storage i ukrywa banner
                    "@click"="localStorage.setItem('cookiesAccepted', 'true'); show = false"
                    class="flex-shrink-0 bg-brand-cyan hover:opacity-80 text-brand-dark font-bold py-2 px-6 rounded-lg transition-opacity"
                {
                    "Rozumiem"
                }
            }
        }
    }
}
