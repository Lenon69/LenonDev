// src/handlers/htmx.rs
use crate::components::sections;
use axum::response::Html;
use maud::{Markup, html};

// Ten handler wyrenderuje obie sekcje i zwróci je jako jeden fragment HTML
pub async fn get_main_content() -> Html<Markup> {
    let markup = html! {
        (sections::about_section())
        (sections::projects_section())
    };
    Html(markup)
}
