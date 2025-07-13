// src/components/layout.rs
use maud::{DOCTYPE, Markup, PreEscaped, html};

fn header_component() -> Markup {
    html! {
        header class="bg-[#1A1A1E]/80 backdrop-blur-sm fixed top-0 left-0 right-0 z-10 border-b border-slate-800/50" {
            nav class="container mx-auto px-4 py-4 flex justify-between items-center" {
                a href="/" {
                    img class="h-10 w-auto" src="./static/LenonDevLogo.png" alt="LenonDev Logo";
                }
                div class="text-slate-200 flex items-center space-x-6" {
                    a href="#projekty" class="text-sm hover:text-brand-cyan transition-colors duration-300" { "Projekty" }
                    a href="#kontakt" class="text-sm hover:text-brand-cyan transition-colors duration-300" { "Kontakt" }
                }
            }
        }
    }
}

fn footer_component() -> Markup {
    html! {
        footer class="relative" {
            div class="absolute top-0 h-px w-full bg-gradient-to-r from-transparent via-brand-cyan/30 to-transparent" {}
            div class="container mx-auto px-4 py-6 text-center text-slate-400" {
                p { "© 2025 LenonDev" }
            }
        }
    }
}

// Funkcja jest teraz publiczna (`pub`), aby była widoczna w innych plikach
pub fn page(title: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="pl" class="scroll-smooth" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                title { (title) }
                link rel="stylesheet" href="/static/main.css";
                link rel="preconnect" href="https://fonts.googleapis.com";
                link rel="preconnect" href="https://fonts.gstatic.com" crossorigin;
                link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;700;900&display=swap" rel="stylesheet";
                style { (PreEscaped("html { font-family: 'Inter', sans-serif; }")) }
                script defer src="https://cdn.jsdelivr.net/npm/alpinejs@3.x.x/dist/cdn.min.js" {}

                // --- NOWOŚĆ: Zabezpieczenie (fallback) ---
                // Jeśli JavaScript (i Alpine.js) jest wyłączony lub się nie załaduje,
                // ten styl sprawi, że ukryte elementy i tak będą widoczne.
                noscript {
                    style {
                        (PreEscaped(".js-invisible { visibility: visible !important; opacity: 1 !important; transform: none !important; }"))
                    }
                }
            }
            body class="bg-brand-dark text-slate-200 antialiased" {
                (header_component())
                main { (content) }
                (footer_component())
            }
        }
    }
}
