use sqlx::PgPool;
/// Struktura, która będzie trzymać stan naszej aplikacji
#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
}
