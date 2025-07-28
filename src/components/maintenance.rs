// src/components/maintenance.rs
use maud::{Markup, html};
use urlencoding::encode;

// Główny widok strony z pakietami opieki
pub fn maintenance_page_view() -> Markup {
    html! {
        // Kontener z animacją wejścia
        div class="container mx-auto px-4 pb-16 lg:pb-24 pt-36 md:pt-28 animate-fade-in" {
            // Nagłówek strony
            div class="text-center mb-16" {
                h1 class="text-4xl lg:text-6xl font-bold tracking-tighter bg-gradient-to-r from-brand-cyan to-brand-green text-transparent bg-clip-text py-4" {
                    "Opieka i Wsparcie Techniczne"
                }
                p class="mt-4 text-lg text-slate-400 max-w-3xl mx-auto" {
                    "Zapewnij swojej stronie ciągłość działania, bezpieczeństwo i najwyższą wydajność. Wybierz pakiet idealnie dopasowany do Twoich potrzeb."
                }
            }

            // Grid z pakietami
            div class="grid grid-cols-1 lg:grid-cols-3 gap-8 max-w-6xl mx-auto" {
                // Pakiet Basic
                (package_card(
                    "Basic",
                    "400 zł",
                    "Idealny dla małych stron-wizytówek i blogów, które potrzebują regularnej opieki.",
                    vec![
                        "Monitoring uptime (24/7)",
                        "Cotygodniowe aktualizacje",
                        "Miesięczna kopia zapasowa",
                        "Podstawowy monitoring bezpieczeństwa",
                        "1 godzina wsparcia technicznego",
                    ],
                    false
                ))

                // Pakiet Standard (wyróżniony)
                (package_card(
                    "Standard",
                    "800 zł",
                    "Najlepszy wybór dla firm, które aktywnie korzystają ze swojej strony i potrzebują niezawodności.",
                    vec![
                        "Wszystko z pakietu Basic",
                        "Codzienna kopia zapasowa",
                        "Zaawansowane skanowanie malware",
                        "Raport wydajności (raz w miesiącu)",
                        "3 godziny wsparcia technicznego",
                    ],
                    true // Ten pakiet będzie wyróżniony
                ))

                // Pakiet Premium
                (package_card(
                    "Premium",
                    "1900 zł",
                    "Kompleksowa obsługa dla sklepów e-commerce i kluczowych aplikacji webowych.",
                    vec![
                        "Wszystko z pakietu Standard",
                        "Monitoring wydajności w czasie rzeczywistym",
                        "Optymalizacja bazy danych",
                        "Wsparcie SEO i konsultacje",
                        "8 godzin wsparcia technicznego",
                    ],
                    false
                ))
            }

            // Sekcja powrotu
            div class="text-center mt-20" {
                a href="/oferta" hx-get="/oferta" hx-target="#content-area" hx-push-url="/oferta" class="cursor-pointer inline-block bg-slate-700 hover:bg-slate-600 transition-colors text-white font-bold py-3 px-8 rounded-lg" {
                    "← Wróć do Głównej Oferty"
                }
            }
        }
    }
}

// Funkcja pomocnicza do tworzenia karty pakietu
fn package_card(
    name: &str,
    price: &str,
    description: &str,
    features: Vec<&str>,
    highlighted: bool,
) -> Markup {
    let message = format!(
        "Dzień dobry, jestem zainteresowany/a Pakietem Opieki nad Stroną: *{}*. Proszę o kontakt w celu omówienia szczegółów i dalszych kroków. Pozdrawiam!",
        name
    );

    // Kodujemy wiadomość, aby była bezpieczna w URL
    let whatsapp_url = format!("https://wa.me/48696619168?text={}", encode(&message));

    // Dynamiczne klasy dla wyróżnionego pakietu
    let border_class = if highlighted {
        "border-brand-cyan shadow-cyan-glow"
    } else {
        "border-slate-700/50"
    };

    let ring_class = if highlighted {
        "ring-2 ring-brand-cyan/50"
    } else {
        ""
    };

    html! {
        div class={"bg-slate-800/50 p-8 rounded-xl border transition-all duration-300 relative overflow-hidden hover:shadow-cyan-glow hover:-translate-y-1" (border_class) " " (ring_class)} {
            // Wyróżniający blask w tle
            @if highlighted {
                div class="absolute top-0 left-0 w-full h-full bg-gradient-to-br from-brand-cyan/10 via-transparent to-brand-purple/10 -z-10" {}
            }

            h3 class="text-2xl font-bold text-center text-white" { (name) }
            div "text-center my-6" {
            p class="text-center text-4xl font-bold my-6 bg-gradient-to-r from-brand-cyan to-brand-green text-transparent bg-clip-text" { (price) }
                p class="text-center text-4xl font-bold my-6 bg-gradient-to-r from-brand-cyan to-brand-green text-transparent bg-clip-text" { (price) }
                p class="text-sm font-normal text-slate-400" { "miesięcznie" }

            }
            p class="text-slate-400 text-sm text-center min-h-[40px]" { (description) }

            // Lista funkcji
            ul class="mt-8 space-y-4" {
                @for feature in features {
                    li class="flex items-start" {
                        svg class="h-6 w-6 text-brand-cyan flex-shrink-0 mr-3" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" {
                            path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7";
                        }
                        span class="text-slate-300" { (feature) }
                    }
                }
            }

            // Przycisk
            div class="text-center mt-10" {
                a href=(whatsapp_url) target="_blank"
                  class={"w-full block text-center font-bold py-3 px-6 rounded-lg transition-all duration-300 " (if highlighted {"bg-brand-cyan text-brand-dark hover:opacity-90"} else {"bg-slate-700 hover:bg-slate-600 text-white"}) } {
                    "Wybieram Pakiet"
                }
            }
        }
    }
}
