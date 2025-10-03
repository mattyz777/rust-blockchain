use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    // Add config fields here
    pub jwt_secret: Arc<String>,
}