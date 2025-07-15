// src/components/layout.rs
use maud::{DOCTYPE, Markup, html};

// Główny szablon, który będzie używany przy pełnym przeładowaniu strony
pub fn base_layout(title: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="pl" class="scroll-smooth" {
            head {
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                title { (title) }

                // Wszystkie skrypty i style z Twojego pliku index.html
                script src="https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4?plugins=typography" {}
                script src="https://cdn.jsdelivr.net/npm/htmx.org@2.0.6/dist/htmx.min.js" {}
                script defer src="https://cdn.jsdelivr.net/npm/alpinejs@3.x.x/dist/cdn.min.js" {}

                // Style (wcześniej w <style> w index.html)
                style type="text/tailwindcss" {
                    "@theme { --color-brand-dark: #101014;
                              --color-brand-purple: #8b5cf6;
                              --color-brand-cyan: #2dd4bf;
                              --color-brand-green: #a3e635;
                              --shadow-cyan-glow: 0 0 15px rgba(45, 212, 191, 0.4), 0 0 25px rgba(45, 212, 191, 0.1); }"

                }
                link rel="preconnect" href="https://fonts.googleapis.com";
                link rel="preconnect" href="https://fonts.gstatic.com" crossorigin;
                link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;700;900&display=swap" rel="stylesheet";
                style {
                    "html { font-family: 'Inter', sans-serif; }"
                }
            }
            // Alpine.js do paska postępu na górze
            body x-data="{ width: 0, update() { let scrollTop = window.scrollY; let docHeight = document.documentElement.scrollHeight - window.innerHeight; this.width = (scrollTop / docHeight) * 100; } }" "@scroll.window"="update()"
                class="bg-brand-dark text-slate-200 antialiased"
            {
                // Pasek postępu
                div class="fixed top-0 left-0 h-1 bg-brand-cyan z-[99]" ":style"="`width: ${width}%`" {}

                // Nagłówek i nawigacja
                header class="bg-[#1A1A1E]/80 backdrop-blur-sm fixed top-0 left-0 right-0 z-50 border-b border-slate-800/50" {
                    nav class="container mx-auto px-4 py-2 flex justify-between items-center" {
                        a href="/" {
                            img class="h-16 w-auto transform transition-transform hover:scale-125" src="/fixed-logo.png" alt="LenonDev Logo";
                        }

                            div class="text-slate-200 flex items-center space-x-6" {
                                a class="cursor-pointer text-sm hover:text-brand-cyan" hx-get="/uses" hx-target="#content-area" hx-push-url="/uses" { "Uses" }
                                a class="cursor-pointer text-sm hover:text-brand-cyan" hx-get="/blog" hx-target="#content-area" hx-push-url="/blog" { "Blog" }

                                a class="cursor-pointer text-sm hover:text-brand-cyan"
                                    // Jeśli jesteśmy na innej stronie, ten atrybut załaduje stronę główną
                                    hx-get="/content"
                                    hx-target="#content-area"
                                    hx-push-url="/"
                                    // A ten atrybut po prostu przewinie do odpowiedniego elementu
                                    href="#projekty"
                                { "Projekty" }

                                a class="cursor-pointer text-sm hover:text-brand-cyan"
                                    hx-get="/content"
                                    hx-target="#content-area"
                                    hx-push-url="/"
                                    href="#kontakt"
                                { "Kontakt" }
                            }
                        }
                    }

                // Główny kontener, w którym będzie renderowana treść
                main id="content-area" {
                    (content)
                }

                // Stopka
                footer class="relative" {
                    div class="absolute top-0 h-px w-full bg-gradient-to-r from-transparent via-brand-cyan/30 to-transparent" {}
                    div class="container mx-auto px-4 py-6 text-center text-slate-400" {
                        a href="/admin/dashboard" class="hover:text-brand-cyan transition-colors" {
                            p { "© 2025 LenonDev" }
                        }
                    }
                }

                // Kontener na modale
                div id="modal-container" {}
            }
        }
    }
}
