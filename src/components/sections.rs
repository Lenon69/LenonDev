// src/components/sections.rs
use maud::{Markup, html};

pub fn about_section() -> Markup {
    html! {
        section id="o-mnie" class="container mx-auto px-4 py-20 lg:py-32 text-center" {
            h2 class="text-3xl lg:text-4xl font-bold tracking-tight" {
                "Pasjonat Kodu i Nowoczesnych Technologii"
            }
            div class="mt-8 max-w-3xl mx-auto text-slate-400 space-y-4 text-lg" {
                p { "Witaj na LenonDev. Programowanie to nie tylko mój zawód, ale przede wszystkim pasja. Eksploruję świat nowoczesnego developmentu, skupiając się na wydajności, bezpieczeństwie i elegancji kodu, którą oferuje ekosystem Rusta." }
                p { "Tworzę strony i aplikacje, które są nie tylko szybkie, ale także gotowe na wyzwania przyszłości." }
            }
            div class="mt-12 flex flex-wrap justify-center items-center gap-x-8 gap-y-4 text-slate-500" {
                span class="font-semibold" { "Używane Technologie:" }
                span { "Rust" }
                span { "Axum" }
                span { "HTMX" }
                span { "PostgreSQL" }
                span { "Tailwind CSS" }
                span { "Alpine.js" }
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
                            span class="font-semibold" { "Technologie:" } {" Axum, Maud, Tailwind CSS"}
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
