use axum::{http::HeaderMap, response::Html};
use maud::Markup;
use moka::sync::Cache;
use resend_rs::Resend;
use sqlx::PgPool;

/// Zunifikowany typ wartości przechowywanej w cache.
pub type CacheValue = (HeaderMap, Html<Markup>);

/// Struktura, która będzie trzymać stan naszej aplikacji
#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    pub resend_client: Resend,
    pub cache: Cache<String, CacheValue>,
}
