use serde::Serialize;
use axum::http::StatusCode;
use axum::Json;

#[derive(Serialize)]
pub struct ApiResponse<T> where T: Serialize {
    pub code: i32,
    pub data: Option<T>,
    pub message: Option<String>,
}

impl<T> ApiResponse<T> where T:Serialize {
    pub fn success(data: Option<T>) -> (StatusCode, Json<ApiResponse<T>>) {
        (
            StatusCode::OK,
            Json(ApiResponse{
                code: 0,
                data,
                message: None,
            })
        )
    }

    pub fn error(code: i32, message: &str) -> (StatusCode, Json<Self>) {
        (
            StatusCode::OK,
            Json(ApiResponse {
                code,
                message: Some(message.to_string()),
                data: None,
            }),
        )
    }
}
