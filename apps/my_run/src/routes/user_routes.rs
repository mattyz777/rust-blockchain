use axum::{
    extract::{State},
    http::StatusCode,
    routing::{post},
    Router,
    Json,
};
use std::sync::atomic::Ordering;
use crate::state::AppState;
use crate::dtos::user_dtos::{UserCreateDTO, UserDTO};

pub fn user_router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_user).get(get_users))
}

#[axum::debug_handler] // important for debugging
async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<UserCreateDTO>,
) -> (StatusCode, Json<UserDTO>) {
    let mut db = state.db.lock().unwrap();

    let new_user = UserDTO {
        id: state.id.fetch_add(1, Ordering::Relaxed) + 1,
        username: payload.username,
    };

    db.push(new_user.clone());

    (StatusCode::CREATED, Json(new_user))
}

#[axum::debug_handler]
async fn get_users(
    State(state): State<AppState>,
    Json(payload): Json<UserCreateDTO>,
) -> (StatusCode, Json<Vec<UserDTO>>) {
    let db = state.db.lock().unwrap();

    let users = db.clone();

    (StatusCode::OK, Json(users))
}