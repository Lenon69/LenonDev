// src/components/sections.rs
use crate::models::{Project, ProjectWithImages};
use maud::{Markup, html};

pub fn about_section() -> Markup {
    html! {
        section id="o-mnie" class="relative bg-cover bg-center"
        style="background-image: url('/public/background.avif');"
        fetchpriority="high"
         {
            div class="absolute inset-0 bg-brand-dark/75" {}
            div class="relative z-10 container mx-auto px-4 pt-36 md:pt-28 pb-20 lg:pb-32 text-center" {
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
                                img class="h-5 w-5" src="/public/svg/rust.svg" alt="Rust logo";
                                span { "Rust" }
                            }

                            // Axum
                            a href="https://github.com/tokio-rs/axum" target="_blank" class="flex items-center gap-2 bg-slate-800/50 border border-slate-700/50 rounded-full px-4 py-2 text-sm text-slate-300 hover:bg-slate-700/50 transition-colors" {
                               img class="h-5 w-5" src="/public/svg/axum.svg" alt="Axum logo";
                               span { "Axum" }
                            }

                            // HTMX
                            a href="https://htmx.org/" target="_blank" class="flex items-center gap-2 bg-slate-800/50 border border-slate-700/50 rounded-full px-4 py-2 text-sm text-slate-300 hover:bg-slate-700/50 transition-colors" {
                                img class="h-5 w-5" src="/public/svg/htmx.svg" alt="HTMX logo";
                                span { "HTMX" }
                            }

                            // PostgreSQL
                            a href="https://www.postgresql.org/" target="_blank" class="flex items-center gap-2 bg-slate-800/50 border border-slate-700/50 rounded-full px-4 py-2 text-sm text-slate-300 hover:bg-slate-700/50 transition-colors" {
                                img class="h-5 w-5" src="/public/svg/postgresql.svg" alt="PostgreSQL logo";
                                span { "PostgreSQL" }
                            }

                            // Tailwind CSS
                            a href="https://tailwindcss.com/" target="_blank" class="flex items-center gap-2 bg-slate-800/50 border border-slate-700/50 rounded-full px-4 py-2 text-sm text-slate-300 hover:bg-slate-700/50 transition-colors" {
                                img class="h-5 w-5" src="/public/svg/tailwind.svg" alt="Tailwind CSS logo";
                                span { "Tailwind CSS" }
                            }

                            // Alpine.js
                            a href="https://alpinejs.dev/" target="_blank" class="flex items-center gap-2 bg-slate-800/50 border border-slate-700/50 rounded-full px-4 py-2 text-sm text-slate-300 hover:bg-slate-700/50 transition-colors" {
                                img class="h-5 w-5" src="/public/svg/alpinejs.svg" alt="Alpine.js logo";
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
        // Krok 1: Zewnętrzny `section` ma teraz tylko tło i pionowy padding.
        section id="projekty" class="bg-[#1A1A1E]/50 py-20 lg:py-32" {
            // Krok 2: Dodajemy `div.container`, który centruje całą zawartość.
            div class="container mx-auto px-4 text-center" {
                h2 class="text-3xl lg:text-4xl font-bold tracking-tight mb-12 text-brand-cyan" { "Aktualne Projekty" }
                div class="grid grid-cols-1 md:grid-cols-2 gap-8 text-left" {
                    @for project in projects {
                        a href=(format!("/projekty/{}", project.slug))
                          class="group block bg-slate-800/50 hover:bg-slate-700/50 p-6 rounded-lg border border-slate-700/50 transition-all duration-300 hover:-translate-y-1 hover:shadow-cyan-glow"
                          hx-get=(format!("/projekty/{}", project.slug))
                          hx-target="#content-area"
                          hx-push-url="true"
                        {
                            @if let Some(image_url) = &project.image_url {
                                img class="w-full h-48 object-cover rounded-md mb-4 group-hover:opacity-80 transition-opacity" src=(image_url) alt=(project.title) loading="lazy" width="300" height="300";
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
                      aria-label="Napisz na WhatsApp"
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

#[allow(dead_code)]
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

pub fn project_detail_page(project: ProjectWithImages) -> Markup {
    // Logika galerii - taka sama jak w starym modalu
    let mut all_images = vec![];
    if let Some(main_image) = &project.image_url {
        all_images.push(main_image.clone());
    }
    all_images.extend(project.images.clone());
    let all_images_json = serde_json::to_string(&all_images).unwrap_or_else(|_| "[]".to_string());

    html! {
        div class="container mx-auto px-4 pb-16 lg:pb-24 pt-36 md:pt-28" {
            // Nagłówek strony projektu
            div class="text-center mb-12" {
                h1 class="text-4xl lg:text-6xl font-bold tracking-tighter bg-gradient-to-r from-brand-cyan to-brand-green text-transparent bg-clip-text py-4" {
                    (project.title)
                }
                p class="mt-4 text-lg text-slate-400" { (project.technologies) }
            }

            // --- GŁÓWNY KONTENER GALERII I MODALA ---
            div class="max-w-5xl mx-auto"
                x-data=(&format!(
                    "{{
                        allImages: {json},
                        currentIndex: 0,
                        modalOpen: false,
                        touchStartX: null, // Używamy null, aby lepiej śledzić stan
                        get mainImage() {{ return this.allImages[this.currentIndex] }},
                        next() {{ this.currentIndex = (this.currentIndex + 1) % this.allImages.length }},
                        prev() {{ this.currentIndex = (this.currentIndex - 1 + this.allImages.length) % this.allImages.length }},
                        openModal(index) {{ this.currentIndex = index; this.modalOpen = true; }},

                        // --- OSTATECZNA, POPRAWIONA LOGIKA SWIPE ---
                        handleTouchStart(event) {{
                            // POPRAWKA 1: Rozpoczynamy gest tylko dla jednego palca
                            if (event.touches.length !== 1) {{
                                this.touchStartX = null; // Resetuj, jeśli to zoom
                                return;
                            }}
                            this.touchStartX = event.touches[0].clientX;
                        }},
                        handleTouchEnd(event) {{
                            // Jeśli gest nigdy się poprawnie nie rozpoczął (np. był to zoom), zakończ
                            if (this.touchStartX === null) return;

                            const touchEndX = event.changedTouches[0].clientX;
                            const distance = touchEndX - this.touchStartX;
                            
                            if (Math.abs(distance) > 50) {{
                                if (distance < 0) {{ this.next(); }} 
                                else {{ this.prev(); }}
                            }}
                            
                            this.touchStartX = null; // Zawsze resetuj na końcu
                        }}
                    }}",
                    json = all_images_json
                ))
                // Nasłuchujemy na klawisz Escape, aby zamknąć modal
                "@keydown.escape.window"="modalOpen = false"
            {
                // --- Galeria na stronie (miniatury i główny obrazek) ---
                div {
                    // Główny obrazek - teraz otwiera modal
                    div class="w-full h-auto overflow-hidden rounded-xl shadow-lg mb-4 cursor-pointer"
                        "@click"="openModal(currentIndex)"
                    {
                        img class="w-full object-cover transition-transform duration-300 hover:scale-105" x-bind:src="mainImage" alt=(project.title);
                    }

                    // Miniatury - teraz otwierają modal z wybranym zdjęciem
                    @if all_images.len() > 1 {
                        div class="flex items-center justify-center space-x-2 mb-12" {
                            template x-for="(image, index) in allImages" {
                                div class="w-20 h-20 rounded-md overflow-hidden cursor-pointer transition-all duration-200"
                                    x-on:click="openModal(index)"
                                    x-bind:class="{ 'border-2 border-brand-cyan scale-110': currentIndex === index, 'opacity-60 hover:opacity-100': currentIndex !== index }"
                                {
                                    img class="h-full w-full object-cover" x-bind:src="image" alt="Thumbnail";
                                }
                            }
                        }
                    }
                }

                // Opis projektu
                div class="prose prose-invert prose-xl mx-auto" {
                    (maud::PreEscaped(project.description))
                }

                // Przyciski
                div class="mt-12 flex justify-center gap-4" {
                    @if let Some(project_url) = &project.project_url {
                        @if project_url != "#" {
                            a href=(project_url) target="_blank" class="inline-block text-center bg-brand-purple hover:opacity-80 transition-opacity text-white font-bold py-3 px-8 rounded-lg" { "Odwiedź stronę" }
                        }
                    }
                    a href="/#projekty" hx-get="/content?scroll_to=projekty" hx-target="#content-area" hx-push-url="/" class="inline-block text-center bg-slate-700 hover:bg-slate-600 transition-colors text-white font-bold py-3 px-8 rounded-lg" { "Wróć do projektów" }
                }



                // --- KOD MODALA ---
                template x-teleport="body" {
                    div x-show="modalOpen"
                        class="fixed inset-0 bg-black/80 backdrop-blur-sm z-[99] flex items-center justify-center p-4"
                        style="display: none;"
                        "@touchstart.passive"="handleTouchStart($event)"
                        "@touchend.passive"="handleTouchEnd($event)"
                    {
                        // Półprzezroczyste tło do zamykania modala
                        div class="absolute inset-0" "@click"="modalOpen = false" {}

                        // Przycisk zamknięcia (X) w prawym górnym rogu
                        button
                            "@click"="modalOpen = false"
                            class="absolute top-4 right-4 text-white hover:text-brand-cyan z-[100] transition-colors"
                            aria-label="Zamknij galerię"
                        {
                            svg class="h-8 w-8" fill="none" viewBox="0 0 24 24" stroke="currentColor" {
                                path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" {}
                            }
                        }

                        // Kontener na obraz i nawigację, który obsługuje gesty swipe
                        div class="relative w-full max-w-6xl flex items-center justify-center" {
                            // Przycisk "Wstecz" (lewa strzałka)
                            button
                                "@click.stop"="prev()"
                                class="absolute left-4 lg:left-0 lg:-translate-x-16 z-10
                                       text-brand-cyan lg:text-white
                                       hover:text-white transition-colors"
                                aria-label="Poprzednie zdjęcie"
                            {
                                svg class="h-10 w-10" fill="none" viewBox="0 0 24 24" stroke="currentColor" {
                                    path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" {}
                                }
                            }

                            // Główny obraz
                            img x-bind:src="mainImage"
                                alt="Powiększone zdjęcie projektu"
                                class="max-h-[95vh] w-auto rounded-lg shadow-2xl select-none"; // `select-none` zapobiega przypadkowemu zaznaczaniu obrazu

                            // Przycisk "Dalej" (prawa strzałka)
                            button
                                "@click.stop"="next()"
                                class="absolute right-4 lg:right-0 lg:translate-x-16 z-10
                                       text-brand-cyan lg:text-white
                                       hover:text-white transition-colors"
                                aria-label="Następne zdjęcie"
                            {
                                svg class="h-10 w-10" fill="none" viewBox="0 0 24 24" stroke="currentColor" {
                                    path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" {}
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
