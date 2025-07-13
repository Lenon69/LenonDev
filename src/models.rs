// src/models.rs
use serde::Serialize;

#[derive(sqlx::FromRow, Serialize)]
pub struct Project {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub technologies: String,
    pub image_url: Option<String>,
    pub project_url: Option<String>,
}
