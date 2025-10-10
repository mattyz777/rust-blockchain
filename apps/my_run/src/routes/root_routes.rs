use axum::{extract::State, Json};
use serde_json::json;
use crate::utils::datetime_utils::get_time;


pub async fn root_router() -> String {
    format!("Hello, World! {}", get_time().await)
}