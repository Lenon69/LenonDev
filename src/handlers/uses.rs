// src/handlers/uses.rs
use axum::{extract::State, http::HeaderMap, response::Html};
use maud::html;

use crate::{
    appstate::{AppState, CacheValue},
    components::layout,
};

/// Kompletna, zoptymalizowana funkcja renderująca zawartość strony /uses.
pub async fn get_uses_content(headers: HeaderMap, State(state): State<AppState>) -> CacheValue {
    let cache_key = "page:/uses".to_string();

    if let Some(cached_page) = state.cache.get(&cache_key) {
        return cached_page;
    }

    let content_fragment = html! {
        // Główny kontener zapewniający odpowiednie marginesy i wyśrodkowanie.
        div class="container mx-auto px-4 pb-16 lg:pb-24 pt-36 md:pt-28" {

            // Nagłówek sekcji.
            div class="text-center mb-12 md:mb-16"{
                h1 class="text-4xl lg:text-5xl font-bold tracking-tighter text-brand-cyan" {"Mój Setup"}
                p class="mt-4 text-lg text-slate-400 max-w-2xl mx-auto" {"Narzędzia, oprogramowanie i usługi, które napędzają moją codzienną pracę."}
            }

            // Siatka (grid) na kategorie - responsywna, z 1, 2 lub 3 kolumnami w zależności od szerokości ekranu.
            div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8"{

                // Karta "Hardware"
                div class="bg-slate-800/50 p-6 lg:p-8 rounded-xl border border-slate-700/50 transition-all duration-300 hover:border-brand-cyan/50 hover:shadow-cyan-glow" {
                    h2 class="text-2xl font-bold text-slate-200 mb-6 border-b border-slate-700 pb-3" {"Hardware"}
                    ul class="space-y-4 text-slate-300" {
                        // Zastosuj nową klasę do wszystkich <li>
                        li class="flex flex-col sm:flex-row sm:justify-between sm:items-baseline" { span class="font-semibold text-slate-400" {"Komputer:"} span {"ASUS ROG Strix G531GW"}}
                        li class="flex flex-col sm:flex-row sm:justify-between sm:items-baseline" { span class="font-semibold text-slate-400" {"Monitor:"} span {"15' Full HD" }}
                        li class="flex flex-col sm:flex-row sm:justify-between sm:items-baseline" { span class="font-semibold text-slate-400" {"Klawiatura:"} span {"ASUS ROG Keyboard"}}
                        li class="flex flex-col sm:flex-row sm:justify-between sm:items-baseline" { span class="font-semibold text-slate-400" {"Mysz:"} span {"Logitech G502"}}
                    }
                }
                // Karta "Software"
                div class="bg-slate-800/50 p-6 lg:p-8 rounded-xl border border-slate-700/50 transition-all duration-300 hover:border-brand-cyan/50 hover:shadow-cyan-glow" {
                    h2 class="text-2xl font-bold text-slate-200 mb-6 border-b border-slate-700 pb-3" {"Software"}
                    ul class="space-y-4 text-slate-300" {
                        li class="flex flex-col sm:flex-row sm:justify-between sm:items-baseline" { span class="font-semibold text-slate-400" {"Edytor:"} span {"Helix Editor"}}
                        li class="flex flex-col sm:flex-row sm:justify-between sm:items-baseline" { span class="font-semibold text-slate-400" {"Terminal:"} span {"PowerShell"}}
                        li class="flex flex-col sm:flex-row sm:justify-between sm:items-baseline" { span class="font-semibold text-slate-400" {"Przeglądarka:"} span {"Brave"}}
                        li class="flex flex-col sm:flex-row sm:justify-between sm:items-baseline" { span class="font-semibold text-slate-400" {"Font:"} span {"Cascadia Mono"}}
                    }
                }

                // Karta "Usługi"
                div class="bg-slate-800/50 p-6 lg:p-8 rounded-xl border border-slate-700/50 transition-all duration-300 hover:border-brand-cyan/50 hover:shadow-cyan-glow" {
                    h2 class="text-2xl font-bold text-slate-200 mb-6 border-b border-slate-700 pb-3" {"Usługi"}
                    ul class="space-y-4 text-slate-300" {
                        li class="flex flex-col sm:flex-row sm:justify-between sm:items-baseline" { span class="font-semibold text-slate-400" {"Hosting:"} span {"OVHcloud"}}
                        li class="flex flex-col sm:flex-row sm:justify-between sm:items-baseline" { span class="font-semibold text-slate-400" {"Kod:"} span {"GitHub"}}
                        li class="flex flex-col sm:flex-row sm:justify-between sm:items-baseline" { span class="font-semibold text-slate-400" {"Baza Danych:"} span {"Neon"}}
                        li class="flex flex-col sm:flex-row sm:justify-between sm:items-baseline" { span class="font-semibold text-slate-400" {"E-mail:"} span {"Resend"}}
                    }
                }
            }
        }
    };

    let page_html = if headers.contains_key("HX-Request") {
        Html(content_fragment)
    } else {
        Html(layout::base_layout(
            "LenonDev - Uses",
            content_fragment,
            None,
            None,
        ))
    };

    let response = (HeaderMap::new(), page_html);
    state.cache.insert(cache_key, response.clone());
    response
}
