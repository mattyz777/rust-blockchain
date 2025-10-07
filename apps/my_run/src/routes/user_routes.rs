use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Router,
    Json,
};
use std::sync::{atomic::Ordering, Arc};
use crate::dtos::user_dtos::{UserDto, UserCreateDto, UserUpdateDto, UserQueryDto};
use crate::AppState;

pub fn user_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(list_users).post(create_user))
        .route("/query", post(query_users))
        .route(
            "/:id",
            get(get_user_by_id).put(update_user).delete(delete_user),
        )
}

// POST /users
async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UserCreateDto>,
) -> (StatusCode, Json<UserDto>) {
    // Note: Storing plaintext passwords is a security risk.
    // For example, using the `bcrypt` crate:
    // let password_hash = bcrypt::hash(&payload.password, bcrypt::DEFAULT_COST).unwrap();
    let encrypted_password = format!("hashed-{}", payload.password);

    let user = UserDto {
        id: state.next_id.fetch_add(1, Ordering::Relaxed) as u64,
        username: payload.username,
        password: encrypted_password,
    };

    state.db.lock().unwrap().push(user.clone());

    (StatusCode::CREATED, Json(user))
}

// PUT /users/:id
async fn update_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<u64>,
    Json(payload): Json<UserUpdateDto>,
) -> Result<Json<UserDto>, StatusCode> {
    let mut users = db.lock().unwrap();
    if let Some(user) = users.iter_mut().find(|u| u.id == id) {
        if let Some(username) = payload.username {
            user.username = username;
        }
        if let Some(password) = payload.password {
            // Again, hash in a real app
            user.password = format!("hashed-{}", password);
        }
        Ok(Json(user.clone()))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

// GET /users/:id
async fn get_user_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<u64>,
) -> Result<Json<UserDto>, StatusCode> {
    let db = state.db.lock().unwrap();
    db.iter()
        .find(|u| u.id == id)
        .map(|u| Json(u.clone()))
        .ok_or(StatusCode::NOT_FOUND)
}

// POST /users/query
async fn query_users(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UserQueryDto>,
) -> Json<Vec<UserDto>> {
    let db = state.db.lock().unwrap();
    let mut filtered_users: Vec<UserDto> = db.clone();

    if let Some(username) = payload.username {
        if !username.is_empty() {
            filtered_users.retain(|u| u.username.contains(&username));
        }
    }
    
    // Note: Filtering by plaintext password is a security risk.
    // This is for demonstration purposes. In a real app, you would not do this.
    if let Some(password) = payload.password {
        if !password.is_empty() {
            let hashed_password = format!("hashed-{}", password);
            filtered_users.retain(|u| u.password == hashed_password);
        }
    }
    
    Json(filtered_users)
}

// GET /users
async fn list_users(State(state): State<Arc<AppState>>) -> Json<Vec<UserDto>> {
    let db = state.db.lock().unwrap();
    Json(db.clone())
}

// DELETE /users/:id
async fn delete_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<u64>,
) -> StatusCode {
    let mut db = state.db.lock().unwrap();
    if let Some(index) = db.iter().position(|u| u.id == id) {
        db.remove(index);
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}