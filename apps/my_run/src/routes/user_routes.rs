use axum::{
    extract::State,
    http::StatusCode,
    routing::post,
    Router,
    Json,
};
use crate::{
    state::AppState,
    dtos::user_dtos::{UserCreateDTO, UserDTO},
    services::user_service::UserService,
    commons::api_response::ApiResponse,
};

pub fn user_router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_user).get(get_users))
        // .route("/{id}", delete(delete_user).get(get_user))
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

// #[axum::debug_handler]
// async fn get_user(
//     State(state): State<AppState>,
//     Path(id): Path<usize>,
// ) -> (StatusCode, Json<Option<UserDTO>>) {
//     // let db = state.db.lock().unwrap();
//     // let user = db.iter().find(|user| user.id == id).cloned();

//     // match user {
//     //     Some(user) => (StatusCode::OK, Json(Some(user))),
//     //     None => (StatusCode::OK, Json(None)),
//     // }
//     (StatusCode::OK, Json(None))
// }

// #[axum::debug_handler]
// async fn delete_user(
//     State(state): State<AppState>,
//     Path(id): Path<usize>,
// ) -> (StatusCode, Json<Option<UserDTO>>) {
//     // let mut db = state.db.lock().unwrap();

//     // if let Some(index) = db.iter().position(|user| user.id == id) {
//     //     let removed_user = db.remove(index);
//     //     (StatusCode::OK, Json(Some(removed_user)))
//     // } else {
//     //     (StatusCode::OK, Json(None))
//     // }
//     (StatusCode::OK, Json(None))
// }