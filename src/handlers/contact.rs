// src/handlers/contact.rs
use axum::{Form, response::Html};
use maud::{Markup, html};
use serde::Deserialize;

// Struktura, która przechwyci dane z formularza
#[derive(Deserialize, Debug)]
pub struct ContactForm {
    email: String,
    message: String,
}

// Handler, który przyjmuje dane z formularza
pub async fn handle_contact_form(Form(form_data): Form<ContactForm>) -> Html<Markup> {
    // Na razie po prostu wyświetlamy dane w konsoli
    println!("Nowa wiadomość od: {}", form_data.email);
    println!("Treść: {}", form_data.message);

    // Zwracamy podziękowanie, które HTMX wstawi na stronę
    let markup = html! {
        div class="p-4 rounded-lg bg-green-800/50 border border-green-700 text-center" {
            p class="font-bold" { "Dziękuję za wiadomość!" }
            p class="text-sm text-slate-300" { "Odezwę się wkrótce." }
        }
    };
    Html(markup)
}
