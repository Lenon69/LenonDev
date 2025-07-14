// src/handlers/blog.rs
use crate::{
    AppState,
    components::{blog, layout, post_page},
    models::{Article, Post},
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

    // Używamy naszego layoutu i widoku listy artykułów
    Html(layout::base_layout(
        "LenonDev - Blog",
        blog::blog_index_view(articles),
    ))
}

// Handler dla /blog/{slug}
pub async fn show_article(State(state): State<AppState>, Path(slug): Path<String>) -> Html<Markup> {
    // TODO: Zaimplementować lepszą obsługę błędów, np. zwracanie 404
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

        // Używamy layoutu i widoku pojedynczego artykułu
        Html(layout::base_layout(
            &article.title.clone(),
            blog::article_detail_view(article, html_output),
        ))
    } else {
        // Prosta obsługa błędu - w przyszłości można tu wstawić ładną stronę 404
        Html(layout::base_layout(
            "Nie znaleziono artykułu",
            html! {
                div class="text-center py-40" {
                    h1 class="text-2xl text-red-500" { "404 - Nie znaleziono artykułu" }
                }
            },
        ))
    }
}

pub async fn get_post_content(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Html<Markup> {
    let post = sqlx::query_as::<_, Post>("SELECT * FROM posts WHERE slug = $1")
        .bind(slug)
        .fetch_one(&state.db_pool)
        .await
        .unwrap();
    Html(post_page::content(post))
}
