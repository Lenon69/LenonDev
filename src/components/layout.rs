// src/components/layout.rs
use maud::{DOCTYPE, Markup, html};

pub fn base_layout(title: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="pl" class="scroll-smooth" {
            head {
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                title { (title) }

                script src="https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4?plugins=typography" {}
                script src="https://cdn.jsdelivr.net/npm/htmx.org@2.0.6/dist/htmx.min.js" {}
                script defer src="https://cdn.jsdelivr.net/npm/alpinejs@3.x.x/dist/cdn.min.js" {}

                style type="text/tailwindcss" {
                    "@theme { --color-brand-dark: #101014; --color-brand-purple: #8b5cf6; --color-brand-cyan: #2dd4bf; --color-brand-green: #a3e635; --shadow-cyan-glow: 0 0 15px rgba(45, 212, 191, 0.4), 0 0 25px rgba(45, 212, 191, 0.1); }"
                }
                link rel="preconnect" href="https://fonts.googleapis.com";
                link rel="preconnect" href="https://fonts.gstatic.com" crossorigin;
                link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;700;900&display=swap" rel="stylesheet";
                style { "html { font-family: 'Inter', sans-serif; }" }
            }
            body
                class="bg-brand-dark text-slate-200 antialiased"
                x-data="{ width: 0, update() { let scrollTop = window.scrollY; let docHeight = document.documentElement.scrollHeight - window.innerHeight; this.width = (scrollTop / docHeight) * 100; } }"
                "@scroll.window"="update()"
            {
                // Wskaźnik przewijania
                div class="fixed top-0 left-0 h-1 bg-brand-cyan z-[99]" ":style"="`width: ${width}%`" {}

                // Nagłówek
                header class="bg-[#1A1A1E]/80 backdrop-blur-sm sticky top-0 left-0 right-0 z-50 border-b border-slate-800/50" {
                    nav class="container mx-auto px-4 py-2 flex justify-between items-center" {
                        a href="/" {
                            img class="h-16 w-auto transform transition-transform hover:scale-125" src="/fixed-logo.png" alt="LenonDev Logo";
                        }
                        div class="text-slate-200 flex items-center space-x-6" {
                            a href="/#projekty" class="text-sm hover:text-brand-cyan transition-colors duration-300" { "Projekty" }
                            // Link do bloga dodamy w ostatnim kroku
                            a href="/blog" class="text-sm hover:text-brand-cyan transition-colors duration-300" { "Blog" }
                            a href="/#kontakt" class="text-sm hover:text-brand-cyan transition-colors duration-300" { "Kontakt" }
                        }
                    }
                }

                // Tutaj wstawiamy właściwą treść strony
                main {
                    (content)
                }

                // Stopka
                footer class="relative" {
                    div class="absolute top-0 h-px w-full bg-gradient-to-r from-transparent via-brand-cyan/30 to-transparent" {}
                    div class="container mx-auto px-4 py-6 text-center text-slate-400" {
                        // Dodajemy link do panelu admina
                        a href="/admin/dashboard" class="hover:text-brand-cyan transition-colors" {
                            "© 2025 LenonDev"
                        }
                    }
                }
                div id="modal-container" {}
            }
        }
    }
}
