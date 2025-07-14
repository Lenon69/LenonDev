// src/handlers/static_pages.rs

// use crate::components::layout; // Dodajemy import naszego modułu layout
use axum::response::Html;
use maud::{Markup, html};

/// Komponent (wygląd) dla strony /uses
fn uses_page_content() -> Markup {
    html! {
        // Kontener dla całej sekcji. Dodajemy pt-20, aby treść nie chowała się pod stałym nagłówkiem
        div class="container mx-auto px-4 py-16 lg:py-24 pt-20" {
            div class="text-center mb-12" {
                h1 class="text-4xl lg:text-5xl font-bold tracking-tight text-brand-cyan" {"Mój setup"}
                p class="mt-4 text-lg text-slate-400" {"Narzędzia i sprzęt, z których korzystam na co dzień."}
            }
            div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8" {
                // Kategoria: Sprzęt
                div class="bg-slate-800/50 p-6 rounded-lg border border-slate-700/50" {
                    h2 class="text-2xl font-bold text-slate-200 mb-4" {"Hardware"}
                    ul class="space-y-2 text-slate-300" {
                        li { strong {" Komputer:"} " - ASUS ROG STRIX 531GW" }
                        li { strong {" Monitor:"} " - ASUS Full HD 1920x1080p" }
                        li { strong {" Klawiatura:"} " - ASUS Keyboard" }
                        li { strong {" Mysz:"} " - Logitech G502" }
                    }
                }
                // Kategoria: Oprogramowanie
                div class="bg-slate-800/50 p-6 rounded-lg border border-slate-700/50" {
                    h2 class="text-2xl font-bold text-slate-200 mb-4" {"Software"}
                    ul class="space-y-2 text-slate-300" {
                        li { strong {" Edytor:"} " - Helix Editor" }
                        li { strong {" Terminal:"} " - PowerShell Customizate" }
                        li { strong {" Przeglądarka:"} " - Brave" }
                        li { strong {" Font:"} " - Cascadia Mono" }
                    }
                }
                // Kategoria: Usługi
                div class="bg-slate-800/50 p-6 rounded-lg border border-slate-700/50" {
                    h2 class="text-2xl font-bold text-slate-200 mb-4" {"Usługi"}
                    ul class="space-y-2 text-slate-300" {
                        li { strong {" Hosting:"} " - Hostinger" }
                        li { strong {" Kod:"} " - GitHub" }
                        li { strong {" Baza Danych:"} " - Neon" }
                        li { strong {" E-mail:"} " - Resend" }
                    }
                }
            }
        }
    }
}

/// Handler, który serwuje stronę /uses
pub async fn uses_page_handler() -> Html<String> {
    // --- POPRAWKA ---
    // Używamy tej samej logiki co na stronie głównej,
    // aby wstawić treść do pełnego szablonu HTML ze stylami.
    let template = layout::page_template("LenonDev - Mój Setup", uses_page_content());
    let html_string = template.into_string();

    let script = layout::get_tailwind_script_and_config();
    let final_html = html_string.replace("", script);

    Html(final_html)
}
