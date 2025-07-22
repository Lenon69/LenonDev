// src/models.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Serialize)]
pub struct Project {
    pub id: i32,
    pub title: String,
    pub slug: String,
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
    pub slug: String,
    pub description: String,
    pub technologies: String,
    pub image_url: Option<String>,
    pub project_url: Option<String>,
    pub images: Vec<String>, // Lista dodatkowych URL-i zdjęć
}

#[derive(Serialize)]
pub struct Publisher<'a> {
    #[serde(rename = "@type")]
    pub type_of: &'a str,
    pub name: &'a str,
    pub logo: ImageObject<'a>,
}

#[derive(Serialize)]
pub struct ImageObject<'a> {
    #[serde(rename = "@type")]
    pub type_of: &'a str,
    pub url: String,
    pub width: u32,
    pub height: u32,
}

#[derive(serde::Serialize)]
pub struct Author {
    #[serde(rename = "@type")]
    pub type_of: String,
    pub name: String,
}

#[derive(Serialize)]
pub struct ArticleSchema<'a> {
    #[serde(rename = "@context")]
    pub context: &'a str,
    #[serde(rename = "@type")]
    pub type_of: &'a str,
    pub headline: String,
    pub author: Author,
    pub image: ImageObject<'a>, // Zmieniamy z Vec<String> na ImageObject
    #[serde(rename = "datePublished")]
    pub date_published: String,
    #[serde(rename = "dateModified")]
    pub date_modified: String,
    pub publisher: Publisher<'a>,
}

#[derive(Serialize)]
pub struct PriceSpecification<'a> {
    #[serde(rename = "@type")]
    pub type_of: &'a str,
    #[serde(rename = "minPrice")]
    pub min_price: &'a str,
    #[serde(rename = "maxPrice")]
    pub max_price: &'a str,
    #[serde(rename = "priceCurrency")]
    pub price_currency: &'a str,
}

#[derive(Serialize)]
pub struct OfferedService<'a> {
    #[serde(rename = "@type")]
    pub type_of: &'a str,
    pub name: &'a str,
    pub description: &'a str,
}

#[derive(Serialize)]
pub struct OfferItem<'a> {
    #[serde(rename = "@type")]
    pub type_of: &'a str,
    #[serde(rename = "itemOffered")]
    pub item_offered: OfferedService<'a>,
    #[serde(rename = "priceSpecification")]
    pub price_specification: PriceSpecification<'a>,
}

#[derive(Serialize)]
pub struct OfferCatalogSchema<'a> {
    #[serde(rename = "@context")]
    pub context: &'a str,
    #[serde(rename = "@type")]
    pub type_of: &'a str,
    pub name: &'a str,
    pub provider: Provider<'a>,
    #[serde(rename = "itemListElement")]
    pub item_list_element: Vec<OfferItem<'a>>,
}

#[derive(Serialize)]
pub struct Provider<'a> {
    #[serde(rename = "@type")]
    pub type_of: &'a str,
    pub name: &'a str,
}

#[derive(Deserialize)]
pub struct ProjectForm {
    pub title: String,
    pub description: String,
    pub technologies: String,
    pub image_url: Option<String>,
    pub project_url: Option<String>,
}

#[derive(sqlx::FromRow)]
pub struct ProjectImage {
    pub id: i32,
    pub image_url: String,
}

#[derive(Deserialize)]
pub struct ProjectUpdateForm {
    pub title: String,
    pub description: String,
    pub technologies: String,
    pub image_url: Option<String>,
    pub project_url: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct PaginationParams {
    #[serde(default = "default_page")]
    pub page: i64,
}

fn default_page() -> i64 {
    1
}
