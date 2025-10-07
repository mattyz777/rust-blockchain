use axum::{
    routing::get,
    Router,
};
use std::sync::Arc;
use crate::AppState;

pub fn auth_routes() -> Router<Arc<AppState>> {
    Router::new().route("/", get(|| async { "Auth route" }))
}
