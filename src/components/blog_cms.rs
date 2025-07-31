// src/components/blog_cms.rs
use crate::components::{
    cta::whatsapp_button,
    simple_site::{back_to_offer_button, feature_card},
};
use maud::{Markup, html};

pub fn blog_cms_page_view() -> Markup {
    html! {
        div class="container mx-auto px-4 pb-16 lg:pb-24 pt-36 md:pt-28 animate-fade-in" {
            div class="text-center mb-16" {
                span class="bg-brand-purple/10 text-brand-purple border border-brand-purple/30 rounded-full px-4 py-1 font-medium" { "Blog / CMS" }
                h1 class="text-4xl lg:text-6xl font-bold tracking-tighter mt-4" { "Dziel Się Wiedzą, Buduj Autorytet" }
                p class="mt-4 text-lg text-slate-400 max-w-3xl mx-auto" {
                    "Strona z systemem zarządzania treścią (CMS) to potężne narzędzie do komunikacji z klientami i budowania pozycji eksperta w swojej dziedzinie."
                }
            }

            div class="max-w-4xl mx-auto" {
                h2 class="text-3xl font-bold text-center text-brand-green mb-10" { "Co zyskujesz?" }
                div class="grid grid-cols-1 md:grid-cols-2 gap-8" {
                    (feature_card(
                        "Prosty Panel Administracyjny",
                        "Zarządzaj treścią bez znajomości kodu. Otrzymasz intuicyjny panel, w którym z łatwością dodasz i edytujesz wpisy na blogu.",
                        "⚙️"
                    ))
                    (feature_card(
                        "Formatowanie Markdown",
                        "Twórz pięknie sformatowane artykuły za pomocą prostego i popularnego języka Markdown, tak jak na moim blogu.",
                        "✍️"
                    ))
                    (feature_card(
                        "Wsparcie dla SEO",
                        "System jest zoptymalizowany pod kątem SEO, z automatycznym generowaniem mapy strony i wsparciem dla meta tagów.",
                        "🚀"
                    ))
                     (feature_card(
                        "Wydajność i Bezpieczeństwo",
                        "W przeciwieństwie do ciężkich systemów jak WordPress, mój autorski CMS jest lekki, szybki i odporny na typowe ataki.",
                        "🛡️"
                    ))
                }
            }

            (whatsapp_button("Strona z Blogiem / CMS"))
            (back_to_offer_button())
        }
    }
}
