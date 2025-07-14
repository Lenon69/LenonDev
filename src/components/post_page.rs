// src/components/post_page.rs
use crate::models::Post; // Zakładamy, że taki model istnieje
use maud::{Markup, PreEscaped, html};

pub fn content(post: Post) -> Markup {
    html! {
        // Kontener dla całego artykułu
        div class="container mx-auto px-4 py-16 lg:py-24" {
            // Używamy klasy `prose` do automatycznego stylowania
            // `prose-invert` dostosowuje kolory do ciemnego tła
            // `max-w-4xl` ogranicza szerokość dla lepszej czytelności
            article class="prose prose-invert max-w-4xl mx-auto" {
                // Nagłówek artykułu
                h1 { (post.title) }
                // Data publikacji
                p class="text-slate-400" { "Opublikowano: " (post.published_date) }

                // Właściwa treść artykułu, renderowana jako surowy HTML
                // To tutaj `prose` zadziała swoją magią
                (PreEscaped(post.content))
            }
        }
    }
}
