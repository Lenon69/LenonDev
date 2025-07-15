// src/models.rs
use chrono::{DateTime, Utc};
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

#[derive(sqlx::FromRow, Serialize, Debug)]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub excerpt: Option<String>,
    pub content: String,
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(serde::Serialize)]
pub struct ProjectWithImages {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub technologies: String,
    pub image_url: Option<String>,
    pub project_url: Option<String>,
    pub images: Vec<String>, // Lista dodatkowych URL-i zdjęć
}

#[derive(sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
}
