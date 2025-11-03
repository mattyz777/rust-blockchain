use axum::{
    extract::{Multipart, State, Path},
    http::StatusCode,
    routing::{delete, post},
    Router,
    Json,
};

use crate::{
    state::AppState,
    dtos::{
        user_dtos::{UserCreateDTO, UserUpdateDTO, UserQueryDTO, UserDTO}, 
        paging_dtos::{PagingDto, PagingQueryDTO},
        response_dto::ResponseDto,
    },
    services::user_service::UserService,
};

pub fn user_router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_user).get(get_users))
        .route("/{id}", delete(delete_user).get(get_user).put(update_user))
        .route("/query", post(query_users))
        .route("/upload", post(upload_file))
}


#[axum::debug_handler]
pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<UserCreateDTO>,
) -> (StatusCode, Json<ResponseDto<UserDTO>>) {
    match UserService::create_user(&state.db, payload).await {
        Ok(user) => ResponseDto::success(Some(user)),
        Err(err) => ResponseDto::error(1001, &format!("failed to create user: {}", err)),
    }
}

#[axum::debug_handler]
pub async fn get_users(
    State(state): State<AppState>,
) -> (StatusCode, Json<ResponseDto<Vec<UserDTO>>>) {
    match UserService::get_users(&state.db).await {
        Ok(users) => ResponseDto::success(Some(users)),
        Err(err) => ResponseDto::error(1001, &format!("failed to get users: {}", err)),
    }
}

#[axum::debug_handler]
pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> (StatusCode, Json<ResponseDto<UserDTO>>) {
    match UserService::get_user(&state.db, id).await {
        Ok(user) => ResponseDto::success(user),
        Err(err) => ResponseDto::error(1001, &format!("failed to get user: {}", err)),
    }
}

#[axum::debug_handler]
pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> (StatusCode, Json<ResponseDto<()>>) {
    match UserService::delete_user(&state.db, id).await {
        Ok(_) => ResponseDto::success(None),
        Err(err) => ResponseDto::error(1001, &format!("failed to delete user: {}", err)),
    }
}

#[axum::debug_handler]
pub async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UserUpdateDTO>,
) -> (StatusCode, Json<ResponseDto<()>>) {
    match UserService::update_user(&state.db, id, payload).await {
        Ok(_) => ResponseDto::success(None),
        Err(err) => ResponseDto::error(1001, &format!("failed to update user: {}", err)),
    }
}

#[axum::debug_handler]
pub async fn query_users(
    State(state): State<AppState>,
    Json(payload): Json<PagingQueryDTO<UserQueryDTO>>,
) -> (StatusCode, Json<ResponseDto<PagingDto<UserDTO>>>) {
    match UserService::query_users(&state.db, payload).await {
        Ok(users) => ResponseDto::success(Some(users)),
        Err(err) => ResponseDto::error(1001, &format!("failed to get users: {}", err)),
    }
}

#[axum::debug_handler]
pub async fn upload_file(
    multipart: Multipart,
) -> (StatusCode, Json<ResponseDto<()>>) {
    match UserService::upload_file(multipart).await {
        Ok(_) => ResponseDto::success(None),
        Err(err) => ResponseDto::error(1001, &format!("failed to upload file: {}", err)),
    }
}