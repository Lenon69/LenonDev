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
    // Używamy Option, bo artykuł może nie być jeszcze opublikowany
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(sqlx::FromRow, serde::Serialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub published_date: String, // Dla uproszczenia jako String
}
