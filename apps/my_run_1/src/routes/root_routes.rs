use axum::{extract::State};
use crate::app_state::AppState;
use crate::utils::date_time_utils::get_time;

pub async fn root_router(State(state): State<AppState>) -> String {
    format!("Hello, World! {}", get_time().await)
}