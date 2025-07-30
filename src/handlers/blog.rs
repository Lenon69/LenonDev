// src/handlers/blog.rs
use crate::{
    AppState,
    appstate::CacheValue,
    components::{blog, cta, layout},
    models::{Article, ArticleSchema, Author, ImageObject, PaginationParams, Publisher},
};
use axum::{
    extract::{Path, State},
    http::{HeaderMap, Uri},
    response::{Html, IntoResponse},
};
use maud::{PreEscaped, html};

// Handler dla /blog
pub async fn blog_index(
    headers: HeaderMap,
    State(state): State<AppState>,
    axum::extract::Query(pagination): axum::extract::Query<PaginationParams>, // Odczytujemy parametr 'page'
) -> CacheValue {
    let is_htmx_request = headers.contains_key("HX-Request");
    let current_page = pagination.page;
    const ARTICLES_PER_PAGE: i64 = 5; // Ile artykułów na stronę?

    // KROK 2: Używamy dynamicznego klucza dla cache'a
    let cache_key = format!("page:/blog?page={}", current_page);

    if !is_htmx_request {
        if let Some(cached_page) = state.cache.get(&cache_key) {
            return cached_page;
        }
    }

    // KROK 3: Pobierz łączną liczbę artykułów
    let total_articles: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM articles WHERE published_at IS NOT NULL")
            .fetch_one(&state.db_pool)
            .await
            .unwrap_or(0);

    let total_pages = (total_articles as f64 / ARTICLES_PER_PAGE as f64).ceil() as i64;
    let offset = (current_page - 1) * ARTICLES_PER_PAGE;

    // KROK 4: Pobierz tylko artykuły dla bieżącej strony
    let articles = sqlx::query_as::<_, Article>(
        "SELECT * FROM articles WHERE published_at IS NOT NULL ORDER BY published_at DESC LIMIT $1 OFFSET $2",
    )
    .bind(ARTICLES_PER_PAGE)
    .bind(offset)
    .fetch_all(&state.db_pool)
    .await
    .unwrap_or_else(|_| vec![]);

    // KROK 5: Przekaż dane o paginacji do widoku
    let content_fragment = blog::blog_index_view(articles, current_page, total_pages);

    if is_htmx_request {
        // Dla HTMX zwracamy tylko fragment, bez zapisywania w cache'u całej strony
        return (HeaderMap::new(), Html(content_fragment));
    }

    // Dla pełnego przeładowania, budujemy całą stronę i zapisujemy w cache'u
    let full_page_html = Html(layout::base_layout(
        "LenonDev - Blog",
        content_fragment,
        Some(
            "Blog o nowoczesnym web developmencie, technologii Rust, Axum, HTMX i tworzeniu wydajnych aplikacji internetowych.",
        ),
        None,
        "/blog",
    ));

    // Stwórz odpowiedź, zapisz ją w cache'u i zwróć
    let response = (HeaderMap::new(), full_page_html);
    state.cache.insert(cache_key, response.clone());

    response
}

// Handler dla /blog/{slug} - W pełni przepisany i zoptymalizowany
pub async fn show_article(
    uri: Uri,
    headers: HeaderMap,
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> impl IntoResponse {
    let is_htmx_request = headers.contains_key("HX-Request");

    // Jeśli to NIE JEST zapytanie HTMX, sprawdzamy cache dla PEŁNEJ STRONY
    if !is_htmx_request {
        let cache_key = format!("page:/blog/{}", slug);
        if let Some(cached_page) = state.cache.get(&cache_key) {
            return cached_page.into_response();
        }
    }

    // Jeśli to zapytanie HTMX lub w cache'u nie ma strony, generujemy treść
    let article_result = sqlx::query_as::<_, Article>(
        "SELECT * FROM articles WHERE slug = $1 AND published_at IS NOT NULL",
    )
    .bind(&slug)
    .fetch_one(&state.db_pool)
    .await;

    match article_result {
        Ok(article) => {
            // Generowanie fragmentu HTML
            let sections: Vec<String> = article
                .content
                .split("---")
                .filter(|s| !s.trim().is_empty())
                .map(|raw_html_section| raw_html_section.trim().to_string()) // Po prostu dzielimy treść na sekcje
                .collect();

            let content_fragment = html! {
                div class="container mx-auto px-4 pb-16 lg:pb-24 pt-36 md:pt-28" {
                    div class="max-w-4xl mx-auto" {
                        div class="text-center mb-12" {
                            @if let Some(published_at) = article.published_at {
                                p class="text-sm text-slate-400 mb-4" { (published_at.format("%d %B %Y")) }
                            }
                            h1 class="text-4xl lg:text-6xl font-bold tracking-tighter bg-gradient-to-r from-brand-cyan to-brand-green text-transparent bg-clip-text py-4" {
                                (article.title)
                            }
                        }
                        @for rendered_html in &sections {
                            section class="py-10" {
                                div class="prose prose-invert prose-xl" { (PreEscaped(rendered_html)) }
                            }
                        }

                        div class="h-px w-full bg-gradient-to-r from-transparent via-brand-purple/30 to-transparent my-12" {}

                        (cta::article_cta())

                        div class="text-center mt-16" {
                            a href="/blog" hx-get="/blog" hx-target="#content-area" hx-push-url="/blog" class="cursor-pointer inline-block bg-slate-700 hover:bg-slate-600 transition-colors text-white font-bold py-2 px-6 rounded-lg" { "← Wróć na bloga" }
                        }
                    }
                }
            };

            // Jeśli to zapytanie HTMX, zwróć tylko fragment
            if is_htmx_request {
                return Html(content_fragment).into_response();
            }

            // W przeciwnym razie, zbuduj całą stronę i zapisz ją w cache'u
            let base_url = std::env::var("APP_BASE_URL").unwrap_or_default();
            let og_image_url = format!("{}/public/og-image.avif", base_url);
            let schema = ArticleSchema {
                context: "https://schema.org",
                type_of: "BlogPosting",
                headline: article.title.clone(),
                image: ImageObject {
                    type_of: "ImageObject",
                    url: og_image_url,
                    width: 1200,
                    height: 630,
                },
                author: Author {
                    type_of: "Person".to_string(),
                    name: "Lenon".to_string(),
                },
                date_published: article
                    .published_at
                    .map(|d| d.to_rfc3339())
                    .unwrap_or_default(),
                publisher: Publisher {
                    type_of: "Organization",
                    name: "LenonDev",
                    logo: ImageObject {
                        type_of: "ImageObject",
                        url: format!("{}/public/fixed-logo.avif", base_url),
                        width: 372,
                        height: 281,
                    },
                },
                date_modified: article.updated_at.to_rfc3339(),
            };
            let schema_json = serde_json::to_string(&schema).ok();

            let full_page_html = Html(layout::base_layout(
                &article.title,
                content_fragment,
                article.excerpt.as_deref(),
                schema_json,
                &format!("/blog/{}", slug),
            ));

            // Zapisz PEŁNĄ stronę w cache'u
            let cache_key = format!("page:/blog/{}", slug);
            state
                .cache
                .insert(cache_key, (HeaderMap::new(), full_page_html.clone()));

            full_page_html.into_response()
        }
        Err(_) => {
            // Tworzymy fragment HTML z informacją o błędzie 404
            let error_content = html! {
                div class="container mx-auto px-4 pb-16 lg:pb-24 pt-36 md:pt-28" {
                    div class="text-center py-40" {
                        h1 class="text-2xl text-red-500" { "404 - Nie znaleziono artykułu" }
                        a href="/blog" hx-get="/blog" hx-target="#content-area" hx-push-url="/blog" class="mt-8 cursor-pointer inline-block bg-slate-700 hover:bg-slate-600 transition-colors text-white font-bold py-2 px-6 rounded-lg" {
                            "← Wróć na bloga"
                        }
                    }
                }
            };

            // Jeśli to zapytanie HTMX, zwróć tylko sam fragment błędu
            if is_htmx_request {
                return Html(error_content).into_response();
            }

            // W przeciwnym razie, zwróć pełną stronę błędu 404 z całym layoutem
            Html(layout::base_layout(
                "404 - Nie znaleziono",
                error_content,
                Some("Strona, której szukasz, nie została znaleziona."),
                None,
                uri.path(),
            ))
            .into_response()
        }
    }
}
