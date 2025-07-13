// src/components/sections.rs
use crate::models::Project;
use maud::{Markup, html};

pub fn about_section() -> Markup {
    html! {
        section id="o-mnie" class="relative bg-cover bg-center" style="background-image: url('/background.jpg');" {
            div class="absolute inset-0 bg-brand-dark/70" {}
            div class="relative z-10 container mx-auto px-4 py-20 lg:py-32 text-center" {
                h2 class="text-3xl lg:text-4xl font-bold tracking-tight" { "Pasjonat Kodu i Nowoczesnych Technologii" }
                div class="mt-8 max-w-3xl mx-auto text-slate-300 space-y-4 text-lg" {
                    p { "Witaj na LenonDev. Programowanie to nie tylko mój zawód, ale przede wszystkim pasja. Eksploruję świat nowoczesnego developmentu, skupiając się na wydajności, bezpieczeństwie i elegancji kodu, którą oferuje ekosystem Rusta." }
                    p { "Tworzę strony i aplikacje, w najnowszych technologiach, które są nie tylko mega szybkie, ale także gotowe na wyzwania przyszłości." }
                }

                    // --- SEKCJA TECHNOLOGII Z ZEWNĘTRZNYMI PLIKAMI SVG ---
                    div class="mt-12" {
                        h3 class="text-lg font-semibold text-slate-300 mb-12" {{"Główny stack technologiczny:"} ;
                        div class="flex flex-wrap justify-center items-center gap-3 lg:gap-4 mt-8" {

                            // Rust - Zastąpiliśmy wbudowane SVG tagiem <img>
                            a href="https://www.rust-lang.org/"
                            target="_blank"
                            class="flex items-center gap-2 bg-slate-800/50 border border-slate-700/50 rounded-full px-4 py-2 text-sm text-slate-300 hover:bg-slate-700/50 transition-colors" {
                                // Dodajemy klasy h-5 (wysokość) i w-5 (szerokość)
                                img class="h-5 w-5" src="/svg/rust.svg" alt="Rust logo";
                                span { "Rust" }
                            }

                            // Axum
                            a href="https://github.com/tokio-rs/axum" target="_blank" class="flex items-center gap-2 bg-slate-800/50 border border-slate-700/50 rounded-full px-4 py-2 text-sm text-slate-300 hover:bg-slate-700/50 transition-colors" {
                               img class="h-5 w-5" src="/svg/axum.svg" alt="Axum logo";
                               span { "Axum" }
                            }

                            // HTMX
                            a href="https://htmx.org/" target="_blank" class="flex items-center gap-2 bg-slate-800/50 border border-slate-700/50 rounded-full px-4 py-2 text-sm text-slate-300 hover:bg-slate-700/50 transition-colors" {
                                img class="h-5 w-5" src="/svg/htmx.svg" alt="HTMX logo";
                                span { "HTMX" }
                            }

                            // PostgreSQL
                            a href="https://www.postgresql.org/" target="_blank" class="flex items-center gap-2 bg-slate-800/50 border border-slate-700/50 rounded-full px-4 py-2 text-sm text-slate-300 hover:bg-slate-700/50 transition-colors" {
                                img class="h-5 w-5" src="/svg/postgresql.svg" alt="PostgreSQL logo";
                                span { "PostgreSQL" }
                            }

                            // Tailwind CSS
                            a href="https://tailwindcss.com/" target="_blank" class="flex items-center gap-2 bg-slate-800/50 border border-slate-700/50 rounded-full px-4 py-2 text-sm text-slate-300 hover:bg-slate-700/50 transition-colors" {
                                img class="h-5 w-5" src="/svg/tailwind.svg" alt="Tailwind CSS logo";
                                span { "Tailwind CSS" }
                            }

                            // Alpine.js
                            a href="https://alpinejs.dev/" target="_blank" class="flex items-center gap-2 bg-slate-800/50 border border-slate-700/50 rounded-full px-4 py-2 text-sm text-slate-300 hover:bg-slate-700/50 transition-colors" {
                                img class="h-5 w-5" src="/svg/alpinejs.svg" alt="Alpine.js logo";
                                span { "Alpine.js" }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn projects_section(projects: Vec<Project>) -> Markup {
    html! {
        section id="projekty" class="bg-[#1A1A1E]/50 py-20 lg:py-32" {
            div class="container mx-auto px-4 text-center" {
                h2 class="text-3xl lg:text-4xl font-bold tracking-tight mb-12" { "Moje Projekty" }
                div class="grid grid-cols-1 md:grid-cols-2 gap-8 text-left" {
                    // Pętla, która renderuje projekty z bazy
                    @for project in projects {
                        a href=(project.project_url.clone().unwrap_or_default()) target="_blank" class="block bg-slate-800/50 hover:bg-slate-700/50 transition-colors p-6 rounded-lg border border-slate-700/50" {
                            @if let Some(image_url) = &project.image_url {
                                img class="w-full h-48 object-cover rounded-md mb-4" src=(image_url) alt=(project.title);
                            }
                            h3 class="text-xl font-bold text-brand-cyan" { (project.title) }
                            p class="text-slate-400 mt-2" { (project.description) }
                            div class="mt-4 text-sm text-slate-500" {
                                span class="font-semibold" { "Technologie:" }
                                " " (project.technologies)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn contact_section() -> Markup {
    html! {
        section id="kontakt" class="container mx-auto px-4 py-20 lg:py-32" {
            div class="max-w-2xl mx-auto text-center" {
                h2 class="text-3xl lg:text-4xl font-bold tracking-tight mb-4" {
                    "Skontaktuj się ze mną"
                }
                p class="text-slate-400" {
                    "Masz pomysł na projekt lub pytanie? Wypełnij formularz, a ja odezwę się najszybciej, jak to możliwe."
                }

                div id="contact-form-response" class="mt-8" {
                    form
                        hx-post="/contact"
                        hx-target="#contact-form-response"
                        hx-swap="innerHTML"
                        class="space-y-4 text-left"
                    {
                        div {
                            label for="email" class="block text-sm font-medium text-slate-300" { "Twój Email" }
                            input
                                type="email"
                                name="email"
                                id="email"
                                required
                                class="mt-1 block w-full bg-slate-800/50 border border-slate-700 rounded-md shadow-sm py-2 px-3 text-white focus:outline-none focus:ring-brand-cyan focus:border-brand-cyan";
                        }
                        div {
                            label for="message" class="block text-sm font-medium text-slate-300" { "Wiadomość" }
                            // --- POPRAWKA ---
                            // Dodajemy puste nawiasy klamrowe {}, aby poprawnie zamknąć tag.
                            textarea
                                name="message"
                                id="message"
                                rows="4"
                                required
                                class="mt-1 block w-full bg-slate-800/50 border border-slate-700 rounded-md shadow-sm py-2 px-3 text-white focus:outline-none focus:ring-brand-cyan focus:border-brand-cyan"
                            {}
                        }
                        div class="text-right" {
                            button
                                type="submit"
                                class="mt-2 inline-block bg-brand-purple hover:shadow-cyan-glow transition-all duration-300 text-white font-bold py-2 px-6 rounded-lg"
                            {
                                "Wyślij"
                            }
                        }
                    }
                }
            }
        }
    }
}
