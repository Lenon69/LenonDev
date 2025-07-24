// src/components/privacy.rs
use maud::{Markup, html};

pub fn privacy_policy_page() -> Markup {
    html! {
        div class="container mx-auto px-4 pb-16 lg:pb-24 pt-36 md:pt-28" {
            div class="max-w-4xl mx-auto prose prose-invert prose-xl" {
                h1 class="text-3xl lg:text-4xl font-bold tracking-tight text-brand-cyan mb-8" { "Polityka dotycząca plików cookies" }

                h2 { "1. Czym są pliki cookies?" }
                p { "Pliki cookies (tzw. „ciasteczka”) to niewielkie pliki tekstowe zapisywane na urządzeniu użytkownika (komputerze, smartfonie, tablecie) podczas przeglądania strony internetowej. Zawierają dane, które ułatwiają korzystanie z witryny, m.in. zapamiętywanie preferencji lub zbieranie anonimowych informacji statystycznych." }

                h2 { "2. Jakich plików cookies używamy?" }
                p { "Nasza strona korzysta wyłącznie z Google Analytics, aby zbierać anonimowe dane statystyczne o sposobie korzystania z witryny, w celu jej udoskonalania." }
                p { "W szczególności:" }
                ul {
                    li { "Pliki cookies Google Analytics pozwalają nam śledzić liczbę odwiedzin, źródła ruchu, czas spędzony na stronie, przeglądane podstrony itp." }
                    li { "Dane te są anonimowe i nie pozwalają na identyfikację tożsamości użytkownika." }
                }

                h2 { "3. Zarządzanie plikami cookies" }
                p { "Użytkownik może w każdej chwili zmienić ustawienia dotyczące plików cookies w swojej przeglądarce internetowej – np. zablokować zapisywanie cookies lub usunąć już zapisane pliki. Należy jednak pamiętać, że wyłączenie cookies może wpłynąć na poprawne działanie niektórych funkcji strony." }
                p { "Więcej informacji na temat zarządzania cookies można znaleźć w dokumentacji przeglądarek:" }
                ul {
                    li { a href="https://support.google.com/chrome" target="_blank" rel="noopener noreferrer" { "Chrome: support.google.com/chrome" } }
                    li { a href="https://support.mozilla.org" target="_blank" rel="noopener noreferrer" { "Firefox: support.mozilla.org" } }
                    li { a href="https://support.microsoft.com" target="_blank" rel="noopener noreferrer" { "Edge: support.microsoft.com" } }
                }

                h2 { "4. Narzędzie Google Analytics" }
                p { "Google Analytics to usługa analityczna świadczona przez Google LLC, z siedzibą w USA. Dane zbierane przez Google Analytics są przetwarzane zgodnie z polityką prywatności Google. Możesz zablokować działanie Google Analytics, instalując wtyczkę do przeglądarki:" }
                p {
                    "👉 "
                    a href="https://tools.google.com/dlpage/gaoptout" target="_blank" rel="noopener noreferrer" { "tools.google.com/dlpage/gaoptout" }
                }

                h2 { "5. Zmiany w polityce cookies" }
                p { "Zastrzegamy sobie prawo do wprowadzania zmian w niniejszej polityce cookies. Wszelkie zmiany będą publikowane na tej stronie." }
                p { "Jeśli masz pytania dotyczące polityki cookies – skontaktuj się z nami poprzez formularz kontaktowy lub e-mail podany w stopce strony." }
            }
        }
    }
}
