// src/components/cta.rs
use maud::{Markup, PreEscaped, html};

pub fn whatsapp_button(service_name: &str) -> Markup {
    let phone_number = "48696619168"; // Twój numer telefonu

    // Profesjonalny, gotowy tekst wiadomości
    let message = format!(
        "Dzień dobry, interesuje mnie usługa: {}. Proszę o informację o możliwym terminie rozpoczęcia współpracy.",
        service_name
    );

    // Kodujemy wiadomość, aby poprawnie działała w linku URL
    let encoded_message = urlencoding::encode(&message);

    let whatsapp_url = format!("https://wa.me/{}?text={}", phone_number, encoded_message);

    html! {
        div class="text-center mt-16" {
            a href=(PreEscaped(&whatsapp_url)) target="_blank"
              class="inline-block no-underline bg-brand-green hover:opacity-90 active:scale-95 text-brand-dark font-bold tracking-wide py-4 px-10 rounded-lg transition-all duration-150 mb-6 text-shadow-sm" {
                "Wybieram Ofertę"
            }
        }
    }
}
pub fn article_cta() -> Markup {
    // Profesjonalny, gotowy tekst wiadomości specjalnie dla czytelników bloga
    let base_message = "Dzień dobry. Piszę w nawiązaniu do artykułu na Pańskiej stronie. Chciałbym/chciałabym omówić szczegóły potencjalnej współpracy. Proszę o kontakt.";
    let encoded_message = urlencoding::encode(base_message);
    let whatsapp_url = format!("https://wa.me/48696619168?text={}", encoded_message);

    html! {
        a href=(whatsapp_url) target="_blank" class="no-underline block" {
            div class="my-6 p-8 text-center border border-brand-cyan/20 bg-brand-dark rounded-xl shadow-cyan-glow transition-transform duration-300 hover:-translate-y-1 cursor-pointer" {
                p class="text-3xl font-bold text-slate-100" { "💡 Masz pomysł na projekt?" }
                p class="mt-4 text-lg text-slate-300" { "✅ Porozmawiajmy o tym, jak technologia może pomóc Twojej firmie rosnąć." }
                div class="mt-8" {
                    div class="inline-flex items-center gap-3 bg-green-500/10 hover:bg-green-500/20 transition-colors duration-300 text-green-400 font-bold py-3 px-8 rounded-lg border border-green-500/30" {
                        svg class="h-6 w-6" fill="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" {
                            path d="M.057 24l1.687-6.163c-1.041-1.804-1.588-3.849-1.587-5.946.003-6.556 5.338-11.891 11.893-11.891 3.181.001 6.167 1.24 8.413 3.488 2.245 2.248 3.481 5.236 3.48 8.414-.003 6.557-5.338 11.892-11.894 11.892-1.99-.001-3.951-.5-5.688-1.448l-6.305 1.654zm6.597-3.807c1.676.995 3.276 1.591 5.392 1.592 5.448 0 9.886-4.434 9.889-9.885.002-5.462-4.415-9.89-9.881-9.892-5.452 0-9.887 4.434-9.889 9.886-.001 2.269.655 4.505 1.905 6.334l-1.196 4.359 4.554-1.187z";
                        }
                        span { "Umów się na bezpłatną konsultację" }
                    }
                }
            }
        }
    }
}
