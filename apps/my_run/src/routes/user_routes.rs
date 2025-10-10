use axum::{
    extract::{State},
    http::StatusCode,
    routing::{post},
    Router,
    Json,
};
use crate::state::AppState;
use crate::dtos::user_dtos::{UserCreateDTO, UserDTO};

pub fn user_router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_user))
}

#[axum::debug_handler] // important for debugging
async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<UserCreateDTO>,
) -> (StatusCode, Json<UserDTO>) {
    let mut db = state.db.lock().unwrap();

    let new_user = UserDTO {
        username: payload.username,
    };

    db.push(new_user.clone());

    (StatusCode::CREATED, Json(new_user))
}