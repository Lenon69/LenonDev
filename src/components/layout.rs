// src/components/layout.rs
use maud::{DOCTYPE, Markup, html};

pub fn base_layout(title: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="pl" class="scroll-smooth" {
            head {
                // Cała zawartość <head> pozostaje bez zmian
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                title { (title) }

                  script {
                    (maud::PreEscaped(r#"
                      tailwind.config = {
                        safelist: [
                          'hover:shadow-cyan-glow',
                          'hover:-translate-y-1',
                          'group-hover:text-brand-cyan',
                          'hover:bg-slate-800/80',
                        ]
                      }
                    "#))
                  }

                script src="https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4?plugins=typography" {}
                script src="https://cdn.jsdelivr.net/npm/htmx.org@2.0.6/dist/htmx.min.js" {}
                script defer src="https://cdn.jsdelivr.net/npm/alpinejs@3.x.x/dist/cdn.min.js" {}
                style type="text/tailwindcss" { "@theme { --color-brand-dark: #101014; --color-brand-purple: #8b5cf6; --color-brand-cyan: #2dd4bf; --color-brand-green: #a3e635; --shadow-cyan-glow: 0 0 15px rgba(45, 212, 191, 0.4), 0 0 25px rgba(45, 212, 191, 0.1); }" }
                link rel="preconnect" href="https://fonts.googleapis.com";
                link rel="preconnect" href="https://fonts.gstatic.com" crossorigin;
                link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;700;900&display=swap" rel="stylesheet";
                style { "html { font-family: 'Inter', sans-serif; }" }
            }
            // --- NOWA, KLUCZOWA ZMIANA W TAGU BODY ---
            body
                // 1. Łączymy logikę paska postępu z istniejącym x-data
                x-data="{
                    width: 0, 
                    update() { 
                        let scrollTop = window.scrollY; 
                        let docHeight = document.documentElement.scrollHeight - window.innerHeight; 
                        this.width = (scrollTop / docHeight) * 100; 
                    } 
                }"
                // 2. Dodajemy nasłuchiwanie na scroll i łączymy je z nasłuchiwaniem na zdarzenie od serwera
                "@scroll.window"="update()"
                "@scroll-to-section.window"="
                    $nextTick(() => {
                        const selector = $event.detail.value;
                        const el = document.querySelector(selector);
                        if (el) {
                            el.scrollIntoView({ behavior: 'smooth' });
                        }
                    })
                "
                class="bg-brand-dark text-slate-200 antialiased"
            {
                // Pasek postępu (bez zmian)
                div class="fixed top-0 left-0 h-1 bg-brand-cyan z-[99]" x-bind:style="`width: ${width}%`" {}

                // Nagłówek i nawigacja
                header class="bg-[#1A1A1E]/80 backdrop-blur-sm fixed top-0 left-0 right-0 z-50 border-b border-slate-800/50" {
                    // Na małych ekranach (domyślnie) elementy będą w kolumnie, a na średnich (md:) i większych w rzędzie.
                    nav class="container mx-auto px-4 py-2 flex flex-col md:flex-row justify-between items-center" {
                        // Link z logo
                        a href="/" {
                        img class="h-16 w-auto transition-transform duration-300 hover:scale-110" src="/fixed-logo.png" alt="LenonDev Logo";
                        }
                        // Linki nawigacji - dodajemy margines górny na małych ekranach (mt-4) i resetujemy go na większych (md:mt-0)
                        div class="text-slate-200 flex flex-wrap justify-center items-center space-x-4 md:space-x-6 mt-4 md:mt-0" {

                            a href="/uses" class="cursor-pointer text-sm hover:text-brand-cyan" { "Uses" }
                            a href="/oferta" class="cursor-pointer text-sm hover:text-brand-cyan" { "Oferta" }
                            a href="/blog" class="cursor-pointer text-sm hover:text-brand-cyan" { "Blog" }

                            a class="cursor-pointer text-sm hover:text-brand-cyan" hx-get="/content?scroll_to=projekty" hx-target="#content-area" hx-push-url="/" { "Projekty" }
                            a class="cursor-pointer text-sm hover:text-brand-cyan" hx-get="/content?scroll_to=kontakt" hx-target="#content-area" hx-push-url="/" { "Kontakt" }
                        }
                    }
                }
                // Reszta strony (bez zmian)
                main id="content-area" class="min-h-screen pt-32 md:pt-24" { (content) }
                footer class="relative" {
                    div class="absolute top-0 h-px w-full bg-gradient-to-r from-transparent via-brand-cyan/30 to-transparent" {}
                    div class="container mx-auto px-4 py-6 text-center text-slate-400" {
                        a href="/admin/dashboard" class="hover:text-brand-cyan transition-colors" { p { "© 2025 LenonDev" } }
                    }
                }
                div id="modal-container" {}
            }
        }
    }
}
