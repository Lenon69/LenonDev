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
            a href=(PreEscaped(&whatsapp_url)) target="_blank" class="inline-block no-underline bg-brand-cyan hover:bg-teal-400 active:scale-95 text-slate-900 font-bold tracking-wide py-4 px-10 rounded-lg transition-all duration-150 mb-6" {
                "Wybieram Ofertę"
            }
        }
    }
}
