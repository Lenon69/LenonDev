// src/components/home_page.rs

use maud::{Markup, html};

pub fn content() -> Markup {
    html! {
        // Usunęliśmy całą logikę animacji z tego kontenera.
        // Teraz treść będzie widoczna od razu.
        section class="min-h-screen flex flex-col items-center justify-center text-center px-4 pt-20" {
            // Nagłówek i opis
            div {
                h1 class="text-5xl md:text-7xl font-black tracking-tighter" {
                    "Tworzę Nowoczesne "
                    br;
                    "Aplikacje Internetowe."
                }
                p class="text-slate-400 mt-6 max-w-2xl mx-auto" {
                    "Specjalizuję się w budowaniu wydajnych i bezpiecznych stron z użyciem najnowszych technologii, takich jak Rust, Axum i HTMX."
                }
            }

            // Przycisk
            div {
                a href="#projekty" class="mt-8 inline-block bg-brand-purple hover:shadow-cyan-glow transition-all duration-300 text-white font-bold py-3 px-8 rounded-lg" {
                    "Zobacz moje prace"
                }
            }
        }
    }
}
