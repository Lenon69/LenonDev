// src/components/post_page.rs
use crate::models::Post;
use maud::{Markup, PreEscaped, html};

pub fn content(post: Post) -> Markup {
    html! {
        div class="container mx-auto px-4 py-16 lg:py-24" {
            article class="prose prose-invert max-w-4xl mx-auto" {
                h1 { (post.title) }
                p class="text-slate-400" { "Opublikowano: " (post.published_date) }
                (PreEscaped(post.content))
            }
        }
    }
}
