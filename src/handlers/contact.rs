// src/handlers/contact.rs
use crate::AppState; // Importujemy AppState
use axum::{Form, extract::State, response::Html};
use maud::{Markup, html};
use resend_rs::types::CreateEmailBaseOptions;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ContactForm {
    email: String,
    message: String,
}

pub async fn handle_contact_form(
    State(state): State<AppState>, // Pobieramy stan aplikacji
    Form(form_data): Form<ContactForm>,
) -> Html<Markup> {
    // Tworzymy treść e-maila
    let email_body = format!(
        "Nowa wiadomość ze strony LenonDev!\n\nOd: {}\n\nTreść:\n{}",
        form_data.email, form_data.message
    );

    // Tworzymy obiekt zapytania do API Resend
    let email_options = CreateEmailBaseOptions::new(
        "onboarding@resend.dev",
        ["fobix.lol@gmail.com"],
        "Nowa wiadomość z formularza LenonDev",
    )
    .with_text(&email_body);

    // Wysyłamy e-mail
    match state.resend_client.emails.send(email_options).await {
        Ok(receipt) => {
            println!("Email wysłany pomyślnie! ID: {}", receipt.id);
            // Zwracamy podziękowanie
            Html(html! {
                div class="text-center" {
                    div class="p-4 rounded-lg bg-green-800/50 border border-green-700" {
                        p class="font-bold" { "Dziękuję za wiadomość!" }
                        p class="text-sm text-slate-300" { "Odezwę się wkrótce." }
                    }
                    // Dodany przycisk "Powrót"
                    a href="/" class="mt-4 inline-block bg-slate-700 hover:bg-slate-600 transition-colors text-white font-bold py-2 px-6 rounded-lg" {
                        "Powrót"
                    }
                }
            })
        }
        Err(e) => {
            println!("Błąd podczas wysyłania emaila: {:?}", e);
            // Zwracamy informację o błędzie
            Html(html! {
                div class="p-4 rounded-lg bg-red-800/50 border border-red-700 text-center" {
                    p class="font-bold" { "Wystąpił błąd." }
                    p class="text-sm text-slate-300" { "Nie udało się wysłać wiadomości. Spróbuj ponownie później." }
                }
            })
        }
    }
}
