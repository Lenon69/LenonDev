// src/handlers/projects.rs
use crate::{
    AppState,
    components::{layout, sections},
    models::{Project, ProjectWithImages},
};
use axum::{
    extract::{Path, State},
    http::HeaderMap,
    response::{Html, IntoResponse},
};
use maud::Markup;

#[allow(dead_code)]
pub async fn get_project_detail(
    State(state): State<AppState>,
    Path(project_id): Path<i32>,
) -> Html<Markup> {
    // 1. Pobieramy główne dane projektu
    let project_result = sqlx::query_as::<_, Project>("SELECT * FROM projects WHERE id = $1")
        .bind(project_id)
        .fetch_one(&state.db_pool)
        .await;

    // Jeśli nie znajdziemy projektu, zwracamy błąd (w przyszłości można tu wstawić ładną stronę 404)
    let project = match project_result {
        Ok(p) => p,
        Err(_) => return Html(maud::html! { h1 { "Nie znaleziono projektu" } }),
    };

    // 2. Pobieramy wszystkie dodatkowe zdjęcia dla tego projektu
    let additional_images = sqlx::query_scalar::<_, String>(
        "SELECT image_url FROM project_images WHERE project_id = $1 ORDER BY id",
    )
    .bind(project_id)
    .fetch_all(&state.db_pool)
    .await
    .unwrap_or_else(|_| vec![]); // Jeśli nie ma zdjęć, zwracamy pustą listę

    // 3. Łączymy wszystko w jedną strukturę
    let project_with_images = ProjectWithImages {
        id: project.id,
        title: project.title,
        slug: project.slug,
        description: project.description,
        technologies: project.technologies,
        image_url: project.image_url,
        project_url: project.project_url,
        images: additional_images,
    };

    // 4. Przekazujemy kompletne dane do szablonu modala
    let markup = sections::project_detail_modal(project_with_images);
    Html(markup)
}

pub async fn show_project(
    headers: HeaderMap,
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> impl IntoResponse {
    let cache_key = format!("fragment:/projekty/{}", slug);
    let is_htmx_request = headers.contains_key("HX-Request");

    // Krok 1: Sprawdź, czy w cache'u istnieje już gotowy FRAGMENT HTML.
    // Zmieniamy typ `CacheValue` na `Html<Markup>`, bo przechowujemy tylko fragmenty.
    let content_fragment: Markup = if let Some(cached_data) = state.cache.get(&cache_key) {
        // Jeśli tak, użyj go.
        cached_data.1.0 // .1 to Html<Markup>, .0 to jego wewnętrzna zawartość
    } else {
        // Jeśli nie, wygeneruj fragment, zapisz go i użyj.
        let project_result = sqlx::query_as::<_, Project>(
            "SELECT id, title, slug, description, technologies, image_url, project_url FROM projects WHERE slug = $1",
        )
        .bind(&slug)
        .fetch_one(&state.db_pool)
        .await;

        match project_result {
            Ok(project) => {
                let additional_images = sqlx::query_scalar::<_, String>(
                    "SELECT image_url FROM project_images WHERE project_id = $1 ORDER BY id",
                )
                .bind(project.id)
                .fetch_all(&state.db_pool)
                .await
                .unwrap_or_else(|_| vec![]);

                let project_with_images = ProjectWithImages {
                    id: project.id,
                    title: project.title.clone(),
                    slug: project.slug.clone(),
                    description: project.description.clone(),
                    technologies: project.technologies.clone(),
                    image_url: project.image_url.clone(),
                    project_url: project.project_url.clone(),
                    images: additional_images,
                };

                let fragment = sections::project_detail_page(project_with_images);

                // Zapisz nowo wygenerowany fragment do cache'a
                state
                    .cache
                    .insert(cache_key, (HeaderMap::new(), Html(fragment.clone())));
                fragment
            }
            Err(_) => {
                // Jeśli projekt nie istnieje, zwróć fragment błędu 404
                maud::html! {
                    div class="text-center py-40" {
                        h1 class="text-2xl text-red-500" { "404 - Nie znaleziono projektu" }
                    }
                }
            }
        }
    };

    // Krok 2: Na samym końcu, na podstawie gotowego fragmentu, zdecyduj, co zwrócić.
    if is_htmx_request {
        // Dla HTMX: zwróć sam fragment.
        Html(content_fragment).into_response()
    } else {
        // Dla pełnego przeładowania: opakuj fragment w pełny layout.
        // Aby uzyskać tytuł, musielibyśmy ponownie odpytać bazę, co jest nieefektywne.
        // Lepszym rozwiązaniem jest ustawienie ogólnego tytułu lub przekazanie go w inny sposób.
        // Na razie ustawmy go na "Projekt".
        let page_title = "Projekt";
        let description = "Szczegóły projektu z portfolio LenonDev."; // Możesz to rozbudować

        Html(layout::base_layout(
            page_title,
            content_fragment,
            Some(description),
            None,
            &format!("/projekty/{}", slug),
        ))
        .into_response()
    }
}
