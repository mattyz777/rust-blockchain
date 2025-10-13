use crate::utils::datetime_utils::get_time;

#[axum::debug_handler] 
pub async fn root_router() -> String {
    format!("Hello, World! {}", get_time())
}