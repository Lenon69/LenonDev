// src/main.rs

use axum::{Router, response::Html, routing::get};
use maud::{DOCTYPE, Markup, html};
use std::net::SocketAddr;

// Krok 1: Definiujemy szablon HTML przy użyciu Maud.
// Zwróć uwagę na składnię `html!`, która jest makrem Maud.
fn root_page() -> Markup {
    html! {
        // DOCTYPE to skrót, który wygeneruje `<!DOCTYPE html>`
        (DOCTYPE)
        html lang="pl" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                title { "LenonDev - Strona w Budowie" }
            }
            body {
                // To jest nasz główny kontener
                div style="display: flex; justify-content: center; align-items: center; height: 100vh; background-color: #111; color: #eee; font-family: sans-serif;" {
                    h1 { "Witaj na stronie LenonDev!" }
                }
            }
        }
    }
}

// Krok 2: Tworzymy "handler" dla Axum.
// Jest to funkcja asynchroniczna, która zostanie wywołana, gdy ktoś wejdzie na główną ścieżkę ("/").
// Zwraca ona `Html`, czyli opakowuje nasz wyrenderowany kod Maud w odpowiedź HTTP.
async fn root_handler() -> Html<String> {
    let page = root_page();
    Html(page.into_string())
}

// Krok 3: Uruchamiamy serwer.
// Atrybut `#[tokio::main]` zamienia naszą funkcję `main` w asynchroniczny runtime Tokio.
#[tokio::main]
async fn main() {
    // Definiujemy router aplikacji - na razie mamy tylko jedną ścieżkę.
    let app = Router::new().route("/", get(root_handler));

    // Definiujemy adres i port, na którym serwer będzie nasłuchiwał.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Serwer nasłuchuje na http://{}", addr);

    // Uruchamiamy serwer.
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
