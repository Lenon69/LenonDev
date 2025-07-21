// src/models.rs
use chrono::{DateTime, Utc};
use serde::Serialize;

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

#[derive(serde::Serialize)]
pub struct Author {
    #[serde(rename = "@type")]
    pub type_of: String,
    pub name: String,
}

#[derive(serde::Serialize)]
pub struct ArticleSchema {
    #[serde(rename = "@context")]
    pub context: String,
    #[serde(rename = "@type")]
    pub type_of: String,
    pub headline: String,
    pub author: Author,
}

#[derive(Serialize)]
struct PriceSpecification<'a> {
    #[serde(rename = "@type")]
    type_of: &'a str,
    #[serde(rename = "minPrice")]
    min_price: &'a str,
    #[serde(rename = "maxPrice")]
    max_price: &'a str,
    #[serde(rename = "priceCurrency")]
    price_currency: &'a str,
}

#[derive(Serialize)]
struct OfferedService<'a> {
    #[serde(rename = "@type")]
    type_of: &'a str,
    name: &'a str,
    description: &'a str,
}

#[derive(Serialize)]
struct OfferItem<'a> {
    #[serde(rename = "@type")]
    type_of: &'a str,
    #[serde(rename = "itemOffered")]
    item_offered: OfferedService<'a>,
    #[serde(rename = "priceSpecification")]
    price_specification: PriceSpecification<'a>,
}

#[derive(Serialize)]
struct OfferCatalogSchema<'a> {
    #[serde(rename = "@context")]
    context: &'a str,
    #[serde(rename = "@type")]
    type_of: &'a str,
    name: &'a str,
    #[serde(rename = "itemListElement")]
    item_list_element: Vec<OfferItem<'a>>,
}
