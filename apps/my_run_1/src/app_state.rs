use std::sync::{Arc, Mutex};
use crate::dtos::user_dtos::UserDTO;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: Arc<Mutex<Vec<UserDTO>>>,
}