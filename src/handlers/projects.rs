// src/handlers/projects.rs
use crate::{
    AppState,
    appstate::CacheValue,
    components::{layout, sections},
    models::{Project, ProjectWithImages},
};
use axum::{
    extract::{Path, State},
    http::HeaderMap,
    response::Html,
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
) -> CacheValue {
    let cache_key = format!("page:/projekty/{}", slug);
    let is_htmx_request = headers.contains_key("HX-Request");

    if let Some(cached_page) = state.cache.get(&cache_key) {
        if is_htmx_request {
            return (HeaderMap::new(), cached_page.1);
        }
        return cached_page;
    }

    let project_result = sqlx::query_as::<_, Project>("SELECT * FROM projects WHERE slug = $1")
        .bind(&slug)
        .fetch_one(&state.db_pool)
        .await;

    let project = match project_result {
        Ok(p) => p,
        Err(_) => {
            // Prosta obsługa błędu 404
            let content = maud::html! { h1 { "404 - Nie znaleziono projektu" }};
            let page_html = if is_htmx_request {
                content.into()
            } else {
                layout::base_layout("404 - Nie znaleziono", content, None, None).into()
            };
            return (HeaderMap::new(), page_html);
        }
    };

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
        slug: project.slug,
        description: project.description,
        technologies: project.technologies,
        image_url: project.image_url,
        project_url: project.project_url,
        images: additional_images,
    };

    let content_fragment = sections::project_detail_page(project_with_images);
    let page_html = if is_htmx_request {
        content_fragment.into()
    } else {
        layout::base_layout(&project.title, content_fragment, None, None).into()
    };

    let response = (HeaderMap::new(), page_html);
    state.cache.insert(cache_key, response.clone());
    response
}
