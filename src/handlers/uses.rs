// src/handlers/uses.rs
use axum::response::Html;
use maud::{Markup, html};

/// Kompletna, zoptymalizowana funkcja renderująca zawartość strony /uses.
pub async fn get_uses_content() -> Html<Markup> {
    let markup = html! {
        // Główny kontener zapewniający odpowiednie marginesy i wyśrodkowanie.
        div class="container mx-auto px-4 py-16 lg:py-24" {

            // Nagłówek sekcji.
            div class="text-center mb-12 md:mb-16"{
                h1 class="text-4xl lg:text-5xl font-bold tracking-tighter text-brand-cyan" {"Mój Setup"}
                p class="mt-4 text-lg text-slate-400 max-w-2xl mx-auto" {"Narzędzia, oprogramowanie i usługi, które napędzają moją codzienną pracę."}
            }

            // Siatka (grid) na kategorie - responsywna, z 1, 2 lub 3 kolumnami w zależności od szerokości ekranu.
            div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8"{

                div class="bg-slate-800/50 p-6 lg:p-8 rounded-xl border border-slate-700/50" {
                    h2 class="text-2xl font-bold text-slate-200 mb-4" {"Hardware"}
                    ul class="space-y-3 text-slate-300" {
                        li class="flex items-baseline" { span class="w-28 font-semibold text-slate-400" {"Komputer:"} {"ASUS ROG Strix G531GW"}
                        li class="flex items-baseline" { span class="w-28 font-semibold text-slate-400" {"Monitor:"} {"ASUS 24\" Full HD"}
                        li class="flex items-baseline" { span class="w-28 font-semibold text-slate-400" {"Klawiatura:"} {"ASUS ROG Keyboard"}
                        li class="flex items-baseline" { span class="w-28 font-semibold text-slate-400" {"Mysz:"} {"Logitech G502"}
                    }
                }

                div class="bg-slate-800/50 p-6 lg:p-8 rounded-xl border border-slate-700/50" {
                    h2 class="text-2xl font-bold text-slate-200 mb-4" {"Software"}
                    ul class="space-y-3 text-slate-300" {
                        li class="flex items-baseline" span class="w-28 font-semibold text-slate-400" {"Edytor:"} {"Helix Editor"}}
                        li class="flex items-baseline" span class="w-28 font-semibold text-slate-400" {"Terminal:"} {"PowerShell"}}
                        li class="flex items-baseline" span class="w-28 font-semibold text-slate-400" {"Przeglądarka:"}{"Brave"}}
                        li class="flex items-baseline" span class="w-28 font-semibold text-slate-400" {"Font:"}{"Cascadia Mono"}}
                    }
                }

                div class="bg-slate-800/50 p-6 lg:p-8 rounded-xl border border-slate-700/50" {
                    h2 class="text-2xl font-bold text-slate-200 mb-4" {"Usługi"}
                    ul class="space-y-3 text-slate-300" {
                        li class="flex items-baseline" span class="w-28 font-semibold text-slate-400" {"Hosting:"}{"Hostinger"}
                        li class="flex items-baseline" span class="w-28 font-semibold text-slate-400" {"Kod:"}{"GitHub"}
                        li class="flex items-baseline" span class="w-28 font-semibold text-slate-400" {"Baza Danych:"}{"Neon"}
                        li class="flex items-baseline" span class="w-28 font-semibold text-slate-400" {"E-mail:"} {"Resend"}
                    }
                }

            }
        }
    };
    Html(markup)
}
