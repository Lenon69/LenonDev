// src/handlers/blog.rs

use crate::{
    AppState,
    components::blog, // Zakładamy, że tutaj są Twoje widoki (view functions)
    models::Article,  // Zakładamy, że masz model Article
};
use axum::{
    extract::{Path, State},
    response::Html,
};
use maud::{Markup, html};

// Handler dla /blog
pub async fn blog_index(State(state): State<AppState>) -> Html<Markup> {
    let articles = sqlx::query_as::<_, Article>(
        "SELECT * FROM articles WHERE published_at IS NOT NULL ORDER BY published_at DESC",
    )
    .fetch_all(&state.db_pool)
    .await
    .unwrap_or_else(|_| vec![]);

    // --- POPRAWKA ---
    // Zwracamy tylko i wyłącznie widok listy artykułów.
    // HTMX wstawi go do <main id="content-area">.
    Html(blog::blog_index_view(articles))
}

// Handler dla /blog/{slug}
pub async fn show_article(State(state): State<AppState>, Path(slug): Path<String>) -> Html<Markup> {
    if let Ok(article) = sqlx::query_as::<_, Article>(
        "SELECT * FROM articles WHERE slug = $1 AND published_at IS NOT NULL",
    )
    .bind(slug)
    .fetch_one(&state.db_pool)
    .await
    {
        let parser = pulldown_cmark::Parser::new(&article.content);
        let mut html_output = String::new();
        pulldown_cmark::html::push_html(&mut html_output, parser);

        // --- POPRAWKA ---
        // Zwracamy tylko i wyłącznie widok szczegółów artykułu.
        Html(blog::article_detail_view(article, html_output))
    } else {
        // --- POPRAWKA ---
        // Zwracamy tylko fragment HTML z informacją o błędzie.
        Html(html! {
            div class="text-center py-40" {
                h1 class="text-2xl text-red-500" { "404 - Nie znaleziono artykułu" }
            }
        })
    }
}
