use serde::Serialize;
use axum::http::StatusCode;
use axum::Json;

#[derive(serde::Serialize)]
pub struct ResponseDto<T> where T: Serialize {
    pub code: i32,
    pub data: Option<T>,
    pub message: Option<String>,
}

impl<T> ResponseDto<T> where T:Serialize {
    pub fn success(data: Option<T>) -> (StatusCode, Json<ResponseDto<T>>) {
        (
            StatusCode::OK,
            Json(ResponseDto{
                code: 1,
                data,
                message: None,
            })
        )
    }

    pub fn error(code: i32, message: &str) -> (StatusCode, Json<Self>) {
        (
            StatusCode::OK,
            Json(ResponseDto {
                code,
                message: Some(message.to_string()),
                data: None,
            }),
        )
    }
}
