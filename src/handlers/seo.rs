use crate::AppState;
use axum::{
    body::Body,
    extract::State,
    http::{HeaderMap, StatusCode, header},
    response::IntoResponse,
};

// Struktury do pobrania danych z bazy
#[derive(sqlx::FromRow)]
struct SitemapUrl {
    slug: String,
    // updated_at: chrono::DateTime<chrono::Utc>, // Możesz dodać to pole dla <lastmod>
}

pub async fn get_sitemap(State(state): State<AppState>) -> impl IntoResponse {
    let base_url = std::env::var("APP_BASE_URL").unwrap_or_default();
    let mut urls = Vec::new();

    // Dodaj statyczne strony
    // --- POCZĄTEK ZMIAN ---
    // Lista wszystkich statycznych ścieżek, które mają znaleźć się w sitemapie
    let static_paths = vec![
        "", // Strona główna
        "/oferta",
        "/blog",
        "/uses",
        "/polityka-prywatnosci",
        "/oferta/prosta-strona-wizytowka",
        "/oferta/landing-page",
        "/oferta/blog-cms",
        "/oferta/sklep-internetowy",
        "/oferta/projekt-indywidualny",
        "/oferta/aplikacja-webowa-crm",
        "/oferta/opieka",
        "/oferta/seo",
        "/oferta/hosting",
    ];

    // Dodaj wszystkie statyczne strony do mapy
    for path in static_paths {
        urls.push(format!("<url><loc>{}{}</loc></url>", base_url, path));
    }
    // --- KONIEC ZMIAN ---

    // Pobierz dynamicznie artykuły
    if let Ok(articles) =
        sqlx::query_as::<_, SitemapUrl>("SELECT slug FROM articles WHERE published_at IS NOT NULL")
            .fetch_all(&state.db_pool)
            .await
    {
        for article in articles {
            urls.push(format!(
                "<url><loc>{}/blog/{}</loc></url>",
                base_url, article.slug
            ));
        }
    }

    // Pobierz dynamicznie projekty
    if let Ok(projects) = sqlx::query_as::<_, SitemapUrl>("SELECT slug FROM projects")
        .fetch_all(&state.db_pool)
        .await
    {
        for project in projects {
            urls.push(format!(
                "<url><loc>{}/projekty/{}</loc></url>",
                base_url, project.slug
            ));
        }
    }

    let sitemap_xml = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?><urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">{}</urlset>"#,
        urls.join("")
    );

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "application/xml".parse().unwrap());

    (StatusCode::OK, headers, Body::from(sitemap_xml))
}
