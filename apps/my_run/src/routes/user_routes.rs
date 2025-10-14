use axum::{
    extract::State,
    http::StatusCode,
    extract::Path,
    routing::{delete, post},
    Router,
    Json,
};
use crate::{
    state::AppState,
    dtos::user_dtos::{UserCreateDTO, UserUpdateDTO, UserDTO},
    services::user_service::UserService,
    commons::api_response::ApiResponse,
};

pub fn user_router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_user).get(get_users))
        .route("/{id}", delete(delete_user).get(get_user).put(update_user))
}


#[axum::debug_handler] // important for debugging
async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<UserCreateDTO>,
) -> (StatusCode, Json<ApiResponse<UserDTO>>) {
    match UserService::create_user(&state.db, payload).await {
        Ok(user) => ApiResponse::success(Some(user)),
        Err(err) => ApiResponse::error(1001, &format!("failed to create user: {}", err)),
    }
}



#[axum::debug_handler]
async fn get_users(
    State(state): State<AppState>,
) -> (StatusCode, Json<ApiResponse<Vec<UserDTO>>>) {
    match UserService::get_users(&state.db).await {
        Ok(users) => ApiResponse::success(Some(users)),
        Err(err) => ApiResponse::error(1001, &format!("failed to get users: {}", err)),
    }
}



#[axum::debug_handler]
async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> (StatusCode, Json<ApiResponse<UserDTO>>) {
    match UserService::get_user(&state.db, id).await {
        Ok(user) => ApiResponse::success(user),
        Err(err) => ApiResponse::error(1001, &format!("failed to get user: {}", err)),
    }
}



#[axum::debug_handler]
async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> (StatusCode, Json<ApiResponse<()>>) {
    match UserService::delete_user(&state.db, id).await {
        Ok(_) => ApiResponse::success(None),
        Err(err) => ApiResponse::error(1001, &format!("failed to delete user: {}", err)),
    }
}



#[axum::debug_handler]
async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UserUpdateDTO>,
) -> (StatusCode, Json<ApiResponse<()>>) {
    match UserService::update_user(&state.db, id, payload).await {
        Ok(_) => ApiResponse::success(None),
        Err(err) => ApiResponse::error(1001, &format!("failed to update user: {}", err)),
    }
}