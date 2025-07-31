// src/components/landing_page.rs
use maud::{Markup, html};

// UWAGA: Zaimportuj funkcje pomocnicze z innego pliku, aby uniknąć duplikacji
use crate::components::{
    cta::whatsapp_button,
    simple_site::{back_to_offer_button, feature_card},
};

pub fn landing_page_view() -> Markup {
    html! {
        div class="container mx-auto px-4 pb-16 lg:pb-24 pt-36 md:pt-28 animate-fade-in" {
            div class="text-center mb-16" {
                span class="bg-brand-purple/10 text-brand-purple border border-brand-purple/30 rounded-full px-4 py-1 font-medium" { "Landing Page" }
                h1 class="text-4xl lg:text-6xl font-bold tracking-tighter mt-4" { "Skupienie na Konwersji" }
                p class="mt-4 text-lg text-slate-400 max-w-3xl mx-auto" {
                    "Landing page to strona zaprojektowana w jednym celu: maksymalizacji konwersji. Idealna do kampanii marketingowych, promocji nowego produktu lub zbierania zapisów na newsletter."
                }
            }

            div class="max-w-4xl mx-auto" {
                h2 class="text-3xl font-bold text-center text-brand-green mb-10" { "Co w wyróżnia skuteczny Landing Page?" }
                div class="grid grid-cols-1 md:grid-cols-2 gap-8" {
                    (feature_card(
                        "Jasny Przekaz (CTA)",
                        "Cała strona, od nagłówka po ostatni przycisk, jest zoptymalizowana pod kątem jednego, konkretnego działania, które ma wykonać użytkownik.",
                        "🎯"
                    ))
                    (feature_card(
                        "Testy A/B",
                        "Przygotowuję różne warianty strony, aby testować, które rozwiązania przynoszą najlepsze rezultaty i najwyższy wskaźnik konwersji.",
                        "🧪"
                    ))
                    (feature_card(
                        "Integracja z Analityką",
                        "Pełna integracja z narzędziami analitycznymi (np. Google Analytics), abyś mógł śledzić skuteczność swoich kampanii w czasie rzeczywistym.",
                        "📊"
                    ))
                     (feature_card(
                        "Brak Rozpraszaczy",
                        "Minimalistyczny design bez zbędnej nawigacji, który prowadzi użytkownika prosto do celu.",
                        "🧘"
                    ))
                }
            }

            (whatsapp_button("Landing Page"))
            (back_to_offer_button())
        }
    }
}
