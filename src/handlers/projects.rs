// src/handlers/projects.rs
use crate::{
    AppState,
    components::sections,
    models::{Project, ProjectWithImages}, // Zaktualizowane importy
};
use axum::{
    extract::{Path, State},
    response::Html,
};
use maud::Markup;

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
        description: project.description,
        technologies: project.technologies,
        image_url: project.image_url,
        project_url: project.project_url,
        images: additional_images, // Przypisujemy listę zdjęć
    };

    // 4. Przekazujemy kompletne dane do szablonu modala
    let markup = sections::project_detail_modal(project_with_images);
    Html(markup)
}
