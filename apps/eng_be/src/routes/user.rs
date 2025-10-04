use axum::{
    extract::{Path, State},
    Json,
    routing::get,
    Router,
};
use std::sync::Arc;
use uuid::Uuid;
use crate::error::AppError;
use crate::AppState;
use crate::dtos::user_dto::{CreateUserRequest, UpdateUserRequest};
use crate::models::user_model::User;

pub fn user_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(list_users).post(create_user))
        .route(
            "/{id}",
            get(get_user).put(update_user).delete(delete_user),
        )
}



// POST /users
#[utoipa::path(
    post,
    path = "/users",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created successfully", body = User),
        (status = 500, description = "Internal server error"),
    ),
    tag = "User API"
)]
async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(axum::http::StatusCode, Json<User>), AppError> {
    let user = User {
        id: Uuid::new_v4(),
        name: payload.name,
        email: payload.email,
    };
    state.db.lock().map_err(|_| AppError::MutexPoisoned)?.push(user.clone());
    // Return 201 Created status on success
    Ok((axum::http::StatusCode::CREATED, Json(user)))
}



// GET /users
#[utoipa::path(
    get,
    path = "/users",
    responses(
        (status = 200, description = "List all users", body = [User]),
        (status = 500, description = "Internal server error"),
    ),
    tag = "User API"
)]
async fn list_users(State(state): State<Arc<AppState>>) -> Result<Json<Vec<User>>, AppError> {
    let db = state.db.lock().map_err(|_| AppError::MutexPoisoned)?;
    Ok(Json(db.clone()))
}



// GET /users/:id
#[utoipa::path(
    get,
    path = "/users/{id}",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User found", body = User),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error"),
    ),
    tag = "User API"
)]
async fn get_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<User>, AppError> {
    let db = state.db.lock().map_err(|_| AppError::MutexPoisoned)?;
    db.iter().find(|u| u.id == id)
        .map(|u| Json(u.clone()))
        .ok_or(AppError::UserNotFound)
}



// PUT /users/:id
#[utoipa::path(
    put,
    path = "/users/{id}",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    request_body = UpdateUserRequest,
    responses(
        (status = 200, description = "User updated successfully", body = User),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error"),
    ),
    tag = "User API"
)]
async fn update_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<User>, AppError> {
    let mut db = state.db.lock().map_err(|_| AppError::MutexPoisoned)?;
    if let Some(user) = db.iter_mut().find(|u| u.id == id) {
        if let Some(name) = payload.name {
            user.name = name;
        }
        if let Some(email) = payload.email {
            user.email = email;
        }
        Ok(Json(user.clone()))
    } else {
        Err(AppError::UserNotFound)
    }
}



// DELETE /users/:id
#[utoipa::path(
    delete,
    path = "/users/{id}",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User deleted successfully", body = String),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error"),
    ),
    tag = "User API"
)]
async fn delete_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<&'static str>, AppError> {
    let mut db = state.db.lock().map_err(|_| AppError::MutexPoisoned)?;
    if let Some(index) = db.iter().position(|u| u.id == id) {
        db.remove(index);
        Ok(Json("User deleted"))
    } else {
        Err(AppError::UserNotFound)
    }
}