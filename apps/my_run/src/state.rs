use std::sync::{
    Arc,
    Mutex,
    atomic::{AtomicUsize},
};
use crate::dtos::user_dtos::UserDTO;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Mutex<Vec<UserDTO>>>,
    pub id: Arc<AtomicUsize>,
}
