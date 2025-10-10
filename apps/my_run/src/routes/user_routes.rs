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

#[axum::debug_handler]
async fn create_user(
    State(state): State<AppState>,
    Json(user_create_dto): Json<UserCreateDTO>,
) -> (StatusCode, Json<UserDTO>) {
    let user = UserDTO {
        name: user_create_dto.name,
    };

    let mut db = state.db.lock().unwrap();
    db.push(user.clone());

    (StatusCode::CREATED, Json(user))
}