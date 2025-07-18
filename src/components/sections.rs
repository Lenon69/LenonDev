// src/components/sections.rs
use crate::models::{Project, ProjectWithImages};
use maud::{Markup, html};

pub fn about_section() -> Markup {
    html! {
        section id="o-mnie" class="relative bg-cover bg-center"
        style="background-image: url('/background.jpg');"
         {
            div class="absolute inset-0 bg-brand-dark/75" {}
            div class="relative z-10 container mx-auto px-4 py-20 lg:py-32 text-center" {
                h2 class="text-3xl lg:text-4xl font-bold tracking-tight text-brand-cyan" { "Pasjonat Kodu i Nowoczesnych Technologii" }
                div class="mt-8 max-w-3xl mx-auto text-slate-300 space-y-4 text-lg" {
                    p { "Witaj na LenonDev. Programowanie to nie tylko mój zawód, ale przede wszystkim pasja. Eksploruję świat nowoczesnego developmentu, skupiając się na wydajności, bezpieczeństwie i elegancji kodu, którą oferuje ekosystem Rusta." }
                    p { "Tworzę strony i aplikacje, w najnowszych technologiach, które są nie tylko super szybkie, ale także gotowe na wyzwania przyszłości." }
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
                h2 class="text-3xl lg:text-4xl font-bold tracking-tight mb-12 text-brand-cyan" { "Aktualne Projekty" }
                div class="grid grid-cols-1 md:grid-cols-2 gap-8 text-left" {
                    @for project in projects {
                        // Zmieniamy <a> na <div> i dodajemy atrybuty HTMX
                        div
                            class="cursor-pointer group bg-slate-800/50 hover:bg-slate-700/50 p-6 rounded-lg border border-slate-700/50 transition-all duration-300 hover:-translate-y-1 hover:shadow-cyan-glow"
                            hx-get=(format!("/project/{}", project.id))
                            hx-target="#modal-container"
                            hx-swap="innerHTML"                        {
                            @if let Some(image_url) = &project.image_url {
                                img class="w-full h-48 object-cover rounded-md mb-4 group-hover:opacity-80 transition-opacity" src=(image_url) alt=(project.title);
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
                h2 class="text-3xl lg:text-4xl font-bold tracking-tight mb-4 text-brand-cyan" {
                    "Skontaktuj się ze mną"
                }
                p class="text-slate-400 mb-12" { // Zwiększony margines dolny
                    "Masz pomysł na projekt lub pytanie? Wypełnij formularz lub napisz na WhatsApp – to najszybsza forma kontaktu."
                }

                // --- NOWA SEKCJA Z PRZYCISKIEM WHATSAPP ---
                div class="mb-12" {
                    a href="https://wa.me/48696619168" target="_blank"
                      class="inline-flex items-center gap-3 bg-green-500/10 hover:bg-green-500/20 transition-colors duration-300 text-green-400 font-bold py-3 px-8 rounded-lg border border-green-500/30" {
                        // Użyjemy SVG dla ikony WhatsApp
                        svg class="h-6 w-6" fill="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" {
                            path d="M.057 24l1.687-6.163c-1.041-1.804-1.588-3.849-1.587-5.946.003-6.556 5.338-11.891 11.893-11.891 3.181.001 6.167 1.24 8.413 3.488 2.245 2.248 3.481 5.236 3.48 8.414-.003 6.557-5.338 11.892-11.894 11.892-1.99-.001-3.951-.5-5.688-1.448l-6.305 1.654zm6.597-3.807c1.676.995 3.276 1.591 5.392 1.592 5.448 0 9.886-4.434 9.889-9.885.002-5.462-4.415-9.89-9.881-9.892-5.452 0-9.887 4.434-9.889 9.886-.001 2.269.655 4.505 1.905 6.334l-1.196 4.359 4.554-1.187z";
                        }
                        span { "Napisz na WhatsApp" }
                    }
                }

                // --- FORMULARZ E-MAIL ---
                div id="contact-form-response" {
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
                                "Wyślij e-mail" // Zmieniony tekst przycisku
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn project_detail_modal(project: ProjectWithImages) -> Markup {
    // Tworzymy jedną, kompletną listę wszystkich zdjęć
    let mut all_images = vec![];
    if let Some(main_image) = &project.image_url {
        all_images.push(main_image.clone());
    }
    all_images.extend(project.images.clone());
    let all_images_json = serde_json::to_string(&all_images).unwrap_or_else(|_| "[]".to_string());

    html! {
        // Inicjalizujemy stan galerii za pomocą Alpine.js
        div x-data=(&format!(
                "{{ open: true, mainImage: '{}', allImages: {} }}",
                all_images.get(0).cloned().unwrap_or_default(),
                all_images_json
            ))
            x-show="open"
            "@keydown.escape.window"="open = false"
            class="fixed inset-0 bg-brand-dark/90 backdrop-blur-sm z-50 flex items-center justify-center p-4"
            style="display: none;"
        {
            // Tło modala (bez zmian)
            div "@click"="open = false" class="absolute inset-0" {}

            // Nowa treść modala z galerią
            div class="relative w-full max-w-4xl bg-slate-800/50 border border-slate-700/50 rounded-lg shadow-2xl flex flex-col md:flex-row overflow-hidden" {

                // === SEKCJA GALERII (LEWA STRONA) ===
                div class="w-full md:w-1/2 flex flex-col bg-brand-dark" {
                    // Główny obraz
                    div class="flex-grow h-64 md:h-auto" {
                        img class="h-full w-full object-cover" x-bind:src="mainImage" alt=(project.title);
                    }
                    // Miniatury (jeśli jest więcej niż 1 zdjęcie)
                    @if all_images.len() > 1 {
                        div class="flex-shrink-0 bg-slate-900/50 p-2" {
                            div class="flex items-center justify-center space-x-2" {
                                template x-for="image in allImages" {
                                    div class="w-16 h-16 flex-shrink-0 rounded-md overflow-hidden cursor-pointer transition-all duration-200"
                                        x-on:click="mainImage = image"
                                        // Dynamiczne podświetlenie aktywnej miniatury
                                        x-bind:class="{ 'border-2 border-brand-cyan scale-110': mainImage === image, 'opacity-60 hover:opacity-100': mainImage !== image }"
                                    {
                                        img class="h-full w-full object-cover" x-bind:src="image" alt="Thumbnail";
                                    }
                                }
                            }
                        }
                    }
                }

                // === SEKCJA SZCZEGÓŁÓW (PRAWA STRONA - BEZ ZMIAN) ===
                div class="w-full md:w-1/2 p-6 lg:p-8 flex flex-col" {
                    h2 class="text-2xl lg:text-3xl font-bold text-brand-cyan" { (project.title) }
                    p class="text-slate-300 mt-4 flex-grow" { (project.description) }
                    div class="mt-6 border-t border-slate-700 pt-4" {
                        h4 class="text-sm font-semibold text-slate-400 mb-2" { "Użyte technologie:" }
                        p class="text-slate-200" { (project.technologies) }
                    }
                    div class="mt-6 flex gap-4" {
                        @if let Some(project_url) = &project.project_url {
                            @if project_url != "#" {
                                a href=(project_url) target="_blank" class="flex-1 text-center bg-brand-purple hover:opacity-80 transition-opacity text-white font-bold py-2 px-4 rounded-lg" { "Odwiedź stronę" }
                            }
                        }
                        button "@click"="open = false" class="flex-1 text-center bg-slate-700 hover:bg-slate-600 transition-colors text-white font-bold py-2 px-4 rounded-lg" { "Zamknij" }
                    }
                }
            }
        }
    }
}
