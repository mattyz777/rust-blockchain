use std::sync::Mutex;
use crate::models::user_model::User;

pub mod dtos;
pub mod error;
pub mod models;
pub mod routes;
pub mod api_docs;
pub mod telemetry;

pub struct AppState {
    pub db: Mutex<Vec<User>>,
}