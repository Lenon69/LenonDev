// src/components/sections.rs
use maud::{Markup, PreEscaped, html};

pub fn about_section() -> Markup {
    html! {
        section id="o-mnie" class="relative bg-cover bg-center" style="background-image: url('/background.jpg');" {
            div class="absolute inset-0 bg-brand-dark/70" {}
            div class="relative z-10 container mx-auto px-4 py-20 lg:py-32 text-center" {
                h2 class="text-3xl lg:text-4xl font-bold tracking-tight" { "Pasjonat Kodu i Nowoczesnych Technologii" }
                div class="mt-8 max-w-3xl mx-auto text-slate-300 space-y-4 text-lg" {
                    p { "Witaj na LenonDev. Programowanie to nie tylko mój zawód, ale przede wszystkim pasja. Eksploruję świat nowoczesnego developmentu, skupiając się na wydajności, bezpieczeństwie i elegancji kodu, którą oferuje ekosystem Rusta." }
                    p { "Tworzę strony i aplikacje, w najnowszych technologiach, które są nie tylko szybkie, ale także gotowe na wyzwania przyszłości." }
                }

                // --- NOWA, ULEPSZONA SEKCJA TECHNOLOGII ---
                div class="mt-12" {
                    h3 class="text-lg font-semibold text-slate-300 mb-6" {"Główny stack technologiczny:"} {
                    div class="flex flex-wrap justify-center items-center gap-3 lg:gap-4" {
                        // Rust
                        a href="https://www.rust-lang.org/"
                        target="_blank"
                        class="flex items-center gap-2 bg-slate-800/50 border border-slate-700/50 rounded-full px-4 py-2 text-sm text-slate-300 hover:bg-slate-700/50 transition-colors" {
                            (PreEscaped(r#"<svg class="h-5 w-5" fill="currentColor" viewBox="0 0 48 48"><path d="M43.55 24H35.4v14.15h-4.25V24h-5.6V18.4h5.6V9.85H35.4V18.4h8.15zM22.5 6.75l-4.25-4.2L4.5 16.3v15.4L18.25 45.4l13.75-13.75V16.3zm-2.85 29.5l-9.9-9.9V16.3l9.9-9.9 9.9 9.9v15.4z"></path></svg>"#))
                            span { "Rust" }
                        }

                        a href="https://github.com/tokio-rs/axum" target="_blank" class="flex items-center gap-2 bg-slate-800/50 border border-slate-700/50 rounded-full px-4 py-2 text-sm text-slate-300 hover:bg-slate-700/50 transition-colors" {
                           (PreEscaped(r#"<svg class="h-5 w-5" fill="currentColor" viewBox="0 0 24 24"><path d="M11.9.5L2.6 5.8v10.7L6.8 9v6.5l-4.2-2.4V8.2l5.2-3 5.2 3v7.9l-5.3 3.1-4.1-2.4v-4l-1.1.6v4.8l5.2 3 9.3-5.3V5.8zm-1.1 11.8l-4-2.3V5.8l4 2.3zm1.1-6.8l4.1 2.4-4.1 2.3-4-2.3z"></path></svg>"#))
                           span { "Axum" }
                        }
                        // HTMX
                        a href="https://htmx.org/" target="_blank" class="flex items-center gap-2 bg-slate-800/50 border border-slate-700/50 rounded-full px-4 py-2 text-sm text-slate-300 hover:bg-slate-700/50 transition-colors" {
                            (PreEscaped(r#"<svg class="h-5 w-5" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" viewBox="0 0 24 24"><path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.72"></path><path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.72-1.72"></path></svg>"#))
                            span { "HTMX" }
                        }
                        // PostgreSQL
                        a href="https://www.postgresql.org/" target="_blank" class="flex items-center gap-2 bg-slate-800/50 border border-slate-700/50 rounded-full px-4 py-2 text-sm text-slate-300 hover:bg-slate-700/50 transition-colors" {
                            (PreEscaped(r#"<svg class="h-5 w-5" fill="currentColor" viewBox="0 0 24 24"><path d="M12.44 22.28a.5.5 0 0 0 .12 0l4-1.25a.5.5 0 0 0 .39-.49v-4.65a.5.5 0 0 0-.5-.5h-2.13v2.87c0 .17-.07.33-.19.45-.12.11-.29.18-.46.18h-2.12v3.34zm6.69-16.85c0-3.32-2.02-4.3-4.32-4.3H3.5a.5.5 0 0 0-.5.5v16.22c0 .28.22.5.5.5h6.63V4.38h3.37v6.63h-2.37a.5.5 0 0 0-.5.5v2.87h2.87c.28 0 .5-.22.5-.5V9.72c2.3 0 4.32-1.1 4.32-4.29zm-4.32-2.13c1.4 0 2.12.56 2.12 2.13s-.72 2.13-2.12 2.13h-3.37V3.3h3.37z"></path></svg>"#))
                            span { "PostgreSQL" }
                        }
                        // Tailwind CSS
                        a href="https://tailwindcss.com/" target="_blank" class="flex items-center gap-2 bg-slate-800/50 border border-slate-700/50 rounded-full px-4 py-2 text-sm text-slate-300 hover:bg-slate-700/50 transition-colors" {
                            (PreEscaped(r#"<svg class="h-5 w-5" fill="currentColor" viewBox="0 0 256 154"><path d="M128.001 0C57.309 0 0 57.309 0 128.001c0 23.336 6.279 45.247 17.544 64.037l-17.38 17.38c-4.881 4.881-4.881 12.796 0 17.677 4.881 4.881 12.796 4.881 17.677 0l17.38-17.38C52.755 221.722 74.665 228 97.964 228c70.692 0 128.001-57.308 128.001-128S198.692 0 128.001 0zm-30.036 103.546a12.799 12.799 0 0 1-18.102 0l-17.545-17.545a12.8 12.8 0 0 1 0-18.102l17.545-17.545a12.8 12.8 0 0 1 18.102 0c4.998 4.998 5.423 13.284.926 18.825-4.498-4.498-12.784-3.999-17.198.416-4.415 4.415-4.415 11.57 0 15.985s11.57 4.415 15.985 0c.99-.99 1.83-2.099 2.49-3.298 5.613 4.57 6.01 13.04.926 18.825zm60.072 0a12.799 12.799 0 0 1-18.102 0l-17.545-17.545a12.8 12.8 0 0 1 0-18.102l17.545-17.545a12.8 12.8 0 0 1 18.102 0c4.998 4.998 5.423 13.284.926 18.825-4.498-4.498-12.784-3.999-17.198.416-4.415 4.415-4.415 11.57 0 15.985s11.57 4.415 15.985 0c.99-.99 1.83-2.099 2.49-3.298 5.613 4.57 6.01 13.04.926 18.825z"></path></svg>"#))
                            span { "Tailwind CSS" }
                        }
                        // Alpine.js
                        a href="https://alpinejs.dev/" target="_blank" class="flex items-center gap-2 bg-slate-800/50 border border-slate-700/50 rounded-full px-4 py-2 text-sm text-slate-300 hover:bg-slate-700/50 transition-colors" {
                            (PreEscaped(r#"<svg class="h-5 w-5" fill="currentColor" viewBox="0 0 24 24"><path d="M11.996 23.36l-3.32-3.32L2.72 14.08V3.32L11.996 12l8.756-8.68V14.08l-5.956 5.96zM11.996 1L3.24 9.756 11.996 18.52l8.756-8.764z"></path></svg>"#))
                            span { "Alpine.js" }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn projects_section() -> Markup {
    html! {
        section id="projekty" class="bg-[#1A1A1E]/50 py-20 lg:py-32" {
            div class="container mx-auto px-4 text-center" {
                h2 class="text-3xl lg:text-4xl font-bold tracking-tight mb-12" {
                    { "Moje Projekty" }
                }
                div class="grid grid-cols-1 md:grid-cols-2 gap-8 text-left" {
                    // Projekt 1
                    div class="bg-slate-800/50 p-6 rounded-lg border border-slate-700/50" {
                        h3 class="text-xl font-bold text-brand-cyan" { "Sklep internetowy vintage online" }
                        p class="text-slate-400 mt-2" { "Nowoczesny, minimalistyczny sklep vintage do sprzedaży produktów, zoptymalizowany pod kątem szybkości ładowania i SEO." }
                        div class="mt-4 text-sm text-slate-500" {
                            span class="font-semibold" { "Technologie:" } {" Rust, Axum, Maud, Tailwind CSS"}
                        }
                    }
                    // Projekt 2
                    div class="bg-slate-800/50 p-6 rounded-lg border border-slate-700/50" {
                        h3 class="text-xl font-bold text-brand-cyan" { "Panel Administracyjny" }
                        p class="text-slate-400 mt-2" { "Interaktywny panel do zarządzania danymi, zbudowany bez przeładowywania strony dzięki HTMX." }
                        div class="mt-4 text-sm text-slate-500" {
                            span class="font-semibold" {"Technologie:"} {" Rust, HTMX, PostgreSQL" }
                        }
                    }
                }
            }
        }
    }
}
