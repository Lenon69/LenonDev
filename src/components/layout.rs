use std::env;

// src/components/layout.rs
use maud::{DOCTYPE, Markup, PreEscaped, html};

use crate::components::cookies;

pub fn base_layout(
    title: &str,
    content: Markup,
    description: Option<&str>,
    schema_json: Option<String>,
    current_path: &str,
) -> Markup {
    let meta_description = description.unwrap_or("LenonDev - Tworzenie nowoczesnych i szybkich stron internetowych w technologii Rust. Pasjonat kodu i nowoczesnych technologii. Tworzę wydajne, bezpieczne i eleganckie strony internetowe. Złoczew");
    let base_url = env::var("APP_BASE_URL").unwrap_or_else(|_| "".to_string());
    let canonical_url = format!("{}{}", base_url, current_path);

    html! {
        (DOCTYPE)
        html lang="pl" class="scroll-smooth" {
            head {
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                script src="https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4?plugins=typography" {}

                script {
                    (maud::PreEscaped(r#"
                        const loadGtm = () => {
                            // Funkcja tworząca i wstrzykująca skrypt GTM
                            const script = document.createElement('script');
                            script.async = true;
                            script.src = 'https://www.googletagmanager.com/gtag/js?id=G-SYZJ42NF6P';
                            document.head.appendChild(script);

                            // Inicjalizacja dataLayer
                            window.dataLayer = window.dataLayer || [];
                            function gtag(){dataLayer.push(arguments);}
                            gtag('js', new Date());
                            gtag('config', 'G-SYZJ42NF6P');

                            // Usuwamy nasłuchiwanie, aby skrypt nie ładował się wielokrotnie
                            ['scroll', 'mousemove', 'touchstart'].forEach(event => 
                                window.removeEventListener(event, loadGtm, { once: true })
                            );
                        };

                        // Nasłuchujemy na pierwszą interakcję użytkownika
                        ['scroll', 'mousemove', 'touchstart'].forEach(event => 
                            window.addEventListener(event, loadGtm, { once: true })
                        );
                    "#))
                }

                // Cała zawartość <head> pozostaje bez zmian
                title { (title) }
                meta name="description" content=(meta_description);
                meta name="keywords" content="tworzenie stron internetowych, strony internetowe na zamówienie, Rust web developer, Axum backend, HTMX frontend, szybkie strony, nowoczesne strony, programista Rust, portfolio programisty, responsywne strony, SEO strony, tani web developer, strony z HTMX, web design, strony dla firm, landing page, strona wizytówka, programowanie stron, zakup strony internetowej, strony pod SEO, profesjonalne strony internetowe, web development Rust, szybkie ładowanie stron, nowoczesne technologie webowe, strony internetowe Złoczew, Złoczew, Sieradz, Wieluń, tworzenie stron internetowych";
                meta name="color-scheme" content="dark only";


                link rel="preload" href="/public/style.css" as="style";
                link rel="preload" href="/public/background.avif" as="image" fetchpriority="high";
                link rel="preload" href="/public/fixed-logo.avif" as="image";
                link rel="preload" href="/public/projects/project1.avif" as="image";
                link rel="preload" href="/public/projects/project2.avif" as="image";

                // Favicon
                link rel="icon" type="image/png" sizes="32x32" href="/public/favicon-32x32.png";
                link rel="icon" type="image/png" sizes="16x16" href="/public/favicon-16x16.png";
                link rel="apple-touch-icon" sizes="180x180" href="/public/apple-touch-icon.png";
                link rel="canonical" href=(canonical_url);


                // Open Graph (dla social media)
                meta property="og:title" content=(title)
                meta property="og:description" content="Tworzę wydajne, bezpieczne i nowoczesne strony internetowe w technologii Rust + HTMX. Postaw na prędkość, SEO i elegancki wygląd.";
                meta property="og:image" content="/public/og-image.avif";
                meta property="og:image:width" content="372";
                meta property="og:image:height" content="281";
                meta property="og:url" content=(canonical_url);
                meta property="og:type" content="website";

                @if let Some(schema) = schema_json {
                    script type="application/ld+json" {
                        (PreEscaped(schema))
                    }
                }

                // --- KONIEC SEKCJI SEO I FAVICON ---
                script defer src="https://cdn.jsdelivr.net/npm/htmx.org@2.0.6/dist/htmx.min.js" {}

                script {
                    (maud::PreEscaped(r#"
                        document.addEventListener('htmx:afterSwap', function(event) {
                            // POPRAWKA: Najpierw sprawdzamy, czy event.detail i event.detail.target istnieją
                            if (event.detail && event.detail.target && event.detail.target.id === 'content-area') {
                                window.scrollTo({
                                    top: 0,
                                    behavior: 'smooth'
                                });
                            }
                        });
                    "#))
                }

                script defer src="https://cdn.jsdelivr.net/npm/alpinejs@3.x.x/dist/cdn.min.js" {}
                style type="text/tailwindcss" {
                    (maud::PreEscaped(r#"
                        @theme {
                            --color-brand-dark: #101014;
                            --color-brand-purple: #8b5cf6;
                            --color-brand-cyan: #2dd4bf;
                            --color-brand-green: #a3e635;
                            --shadow-cyan-glow: 0 0 15px rgba(45, 212, 191, 0.4), 0 0 25px rgba(45, 212, 191, 0.1);
                        }

                        @source inline("hover:shadow-cyan-glow hover:-translate-y-1 group-hover:text-brand-cyan hover:bg-slate-800/80");
                    "#))
                }
                link rel="stylesheet" href="/public/style.css";
                style { "html { font-family: 'Inter', sans-serif; }" }
            }
            // --- NOWA, KLUCZOWA ZMIANA W TAGU BODY ---
            body
                // 1. Łączymy logikę paska postępu z istniejącym x-data
                x-data="{
                    width: 0, 
                    update() { 
                        let scrollTop = window.scrollY; 
                        let docHeight = document.documentElement.scrollHeight - window.innerHeight; 
                        this.width = (scrollTop / docHeight) * 100; 
                    } 
                }"
                // 2. Dodajemy nasłuchiwanie na scroll i łączymy je z nasłuchiwaniem na zdarzenie od serwera
                "@scroll.window"="update()"
                "@scroll-to-section.window"="
                    // Używamy $nextTick, aby mieć pewność, że DOM jest już gotowy
                    $nextTick(() => {
                        const selector = $event.detail.value;
                        const el = document.querySelector(selector);
        
                        if (el) {
                            const header = document.getElementById('main-header');
                            const headerHeight = header ? header.offsetHeight : 0;

                            const elementPosition = el.getBoundingClientRect().top + window.scrollY;
                            const offsetPosition = elementPosition - headerHeight;

                            window.scrollTo({
                                top: offsetPosition,
                                behavior: 'smooth'
                            });
                        }
                    })
                "
                class="bg-brand-dark text-slate-200 antialiased"
            {

                noscript {
                    iframe
                        src="https://www.googletagmanager.com/ns.html?id=GTM-N6HHSWLP"
                        height="0"
                        width="0"
                        style="display:none;visibility:hidden" {}
                }
                // Pasek postępu (bez zmian)
                div class="fixed top-0 left-0 h-1 bg-brand-cyan z-[99]" x-bind:style="`width: ${width}%`" {}

                // Nagłówek i nawigacja
                header id="main-header" class="bg-[#1A1A1E]/80 backdrop-blur-sm fixed top-0 left-0 right-0 z-50 border-b border-slate-800/50" {
                    // Na małych ekranach (domyślnie) elementy będą w kolumnie, a na średnich (md:) i większych w rzędzie.
                    nav class="container mx-auto px-4 py-2 flex flex-col md:flex-row justify-between items-center" {
                        // Link z logo
                        a href="/"
                          hx-get="/content"
                          hx-target="#content-area"
                          hx-push-url="/"
                        {
                            img class="h-16 w-auto transition-transform duration-300 hover:scale-110" src="/public/fixed-logo.avif" alt="LenonDev Logo" width="372" height="281";
                        }
                        // Linki nawigacji - dodajemy margines górny na małych ekranach (mt-4) i resetujemy go na większych (md:mt-0)
                        div class="text-slate-200 flex flex-wrap justify-center items-center gap-2 md:gap-4 mt-4 md:mt-0" {
                        a href="/uses"   class="cursor-pointer text-sm hover:text-brand-cyan px-1 py-2 rounded-md transition-all duration-300 hover:shadow-cyan-glow" { "Uses" }
                        a href="/oferta" class="cursor-pointer text-sm hover:text-brand-cyan px-1 py-2 rounded-md transition-all duration-300 hover:shadow-cyan-glow" { "Oferta" }
                        a href="/blog"   class="cursor-pointer text-sm hover:text-brand-cyan px-1 py-2 rounded-md transition-all duration-300 hover:shadow-cyan-glow" { "Blog" }
                        a href="/#projekty" hx-get="/content?scroll_to=projekty" hx-target="#content-area" hx-push-url="/" class="cursor-pointer text-sm hover:text-brand-cyan px-1 py-2 rounded-md transition-all duration-300 hover:shadow-cyan-glow" { "Projekty" }
                        a href="/#kontakt" hx-get="/content?scroll_to=kontakt"  hx-target="#content-area" hx-push-url="/" class="cursor-pointer text-sm hover:text-brand-cyan px-1 py-2 rounded-md transition-all duration-300 hover:shadow-cyan-glow" { "Kontakt" }
                        }
                    }
                }
                // Reszta strony (bez zmian)
                main id="content-area" class="min-h-screen" { (content) }
                footer class="relative" {
                    div class="absolute top-0 h-px w-full bg-gradient-to-r from-transparent via-brand-cyan/30 to-transparent" {}
                    div class="container mx-auto px-4 py-6 text-center text-slate-400 flex flex-col sm:flex-row justify-center items-center gap-x-4 gap-y-2" {
                            a href="/admin/dashboard" class="hover:text-brand-cyan transition-colors" { p { "© 2025 LenonDev" } }
                            span class="hidden sm:inline" { "|" }
                            a href="/polityka-prywatnosci" class="hover:text-brand-cyan transition-colors" { "Polityka Cookies" }
                        }
                    }
                div id="modal-container" {}


                (cookies::cookies_banner())
            }
        }
    }
}
