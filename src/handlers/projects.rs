// src/handlers/projects.rs
use crate::{AppState, components::sections, models::Project};
use axum::{
    extract::{Path, State},
    response::Html,
};
use maud::Markup;

pub async fn get_project_detail(
    State(state): State<AppState>,
    Path(project_id): Path<i32>,
) -> Html<Markup> {
    let project = sqlx::query_as::<_, Project>("SELECT * FROM projects WHERE id = $1")
        .bind(project_id)
        .fetch_one(&state.db_pool)
        .await
        .unwrap(); // W prawdziwej aplikacji obsłuż błąd!

    let markup = sections::project_detail_modal(project);
    Html(markup)
}
