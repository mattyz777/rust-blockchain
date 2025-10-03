use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};


pub enum AppError {
    UserNotFound,
    MutexPoisoned,
}


impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::UserNotFound => (StatusCode::NOT_FOUND, "User not found"),
            AppError::MutexPoisoned => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
        };
        (status, error_message).into_response()
    }
}