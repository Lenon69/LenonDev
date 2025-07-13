// src/handlers/htmx.rs
use crate::{AppState, components::sections, models::Project}; // Dodajemy importy
use axum::{extract::State, response::Html};
use maud::{Markup, html};

pub async fn get_main_content(State(state): State<AppState>) -> Html<Markup> {
    // Pobieramy projekty z bazy danych
    let projects = sqlx::query_as::<_, Project>("SELECT * FROM projects")
        .fetch_all(&state.db_pool)
        .await
        .unwrap_or_else(|e| {
            println!("Failed to fetch projects: {}", e);
            vec![]
        });

    let markup = html! {
        (sections::about_section())
        (sections::projects_section(projects))
        (sections::contact_section())
    };
    Html(markup)
}
