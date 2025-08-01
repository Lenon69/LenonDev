use maud::{Markup, html};

pub fn footer_with_map() -> Markup {
    html! {
        footer class="relative" {
            // Separator
            div class="absolute top-0 h-px w-full bg-gradient-to-r from-transparent via-brand-cyan/30 to-transparent" {}

            // Sekcja z mapą
            div class="bg-slate-900/50 py-12" {
                div class="container mx-auto px-4 text-center" {
                    // Kontener na mapę z logiką Alpine.js
                    div
                        x-data="{ showMap: false }"
                        class="relative max-w-4xl mx-auto aspect-[16/9] rounded-lg overflow-hidden border-2 border-slate-700/50"
                    {
                        // 1. Statyczny obrazek (fasada) - widoczny na początku
                        img
                            x-show="!showMap"
                            src="/public/map-placeholder.avif" // Pamiętaj o stworzeniu tego pliku
                            alt="Mapa lokalizacji firmy LenonDev"
                            loading="lazy"
                            class="w-full h-full object-cover cursor-pointer";

                        // Nakładka z przyciskiem do aktywacji mapy
                        div
                            x-show="!showMap"
                            "@click"="showMap = true"
                            class="absolute inset-0 bg-black/50 flex items-center justify-center cursor-pointer transition-opacity hover:opacity-80"
                        {
                            div class="text-center bg-brand-dark/80 backdrop-blur-sm p-4 rounded-lg border border-slate-700" {
                                p class="font-bold text-white" { "📍 Pokaż interaktywną mapę" }
                                p class="text-sm text-slate-400" { "Mapa załaduje się po kliknięciu" }
                            }
                        }

                        // 2. Interaktywna mapa (iframe) - ładowana po kliknięciu
                        template x-if="showMap" {
                            iframe
                                src="https://www.google.com/maps/embed?pb=!1m18!1m12!1m3!1d2488.5512931743046!2d18.570327040272286!3d51.411298890853516!2m3!1f0!2f0!3f0!3m2!1i1024!2i768!4f13.1!3m3!1m2!1s0x471a89528422a34f%3A0x72c37c576167d40b!2sLenonDev!5e0!3m2!1spl!2spl!4v1754058478110!5m2!1spl!2spl"
                                class="w-full h-full"
                                allowfullscreen=""
                                loading="lazy"
                                referrerpolicy="no-referrer-when-downgrade" {}
                        }
                    }
                }
            }

            // Twoja obecna stopka z linkami
            div class="container mx-auto px-4 py-6 text-center text-slate-400 flex flex-col sm:flex-row justify-center items-center gap-x-4 gap-y-2" {
                a href="/admin/dashboard" class="hover:text-brand-cyan transition-colors" {
                    p { "© 2025 LenonDev" }
                }
                span class="hidden sm:inline" { "|" }
                a href="/polityka-prywatnosci" class="hover:text-brand-cyan transition-colors" {
                    "Polityka Cookies"
                }
            }
        }
    }
}
