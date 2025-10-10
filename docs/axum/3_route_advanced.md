# structure

```
├── Cargo.toml
└── src
    ├── main.rs
    ├── lib.rs
    ├── state.rs
    |
    ├── dtos
    │   ├── auth_dtos.rs
    │   ├── mod.rs
    │   └── user_dtos.rs
    |
    ├── utils
    │   ├── mod.rs
    │   └── date_time_utils.rs
    |
    └── routes
        ├── auth.rs
        ├── mod.rs
        └── user.rs
```

# Cargo.toml

```toml
[dependencies]
# Web Framework
axum = { version = "0.8.5", features = ["macros"] }
tokio = { version = "1.47.1", features = ["full"] }

chrono = { version = "0.4.42", features = ["serde"] }
serde = { version = "1.0.228", features = ["derive"] }
serde_json = "1.0.145"

```

# app_state.rs

```rs
use std::sync::{Arc, Mutex};
use crate::dtos::user_dtos::UserDTO;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: Arc<Mutex<Vec<UserDTO>>>,
}
```

# routes/root_routes.rs

```rs
use axum::{extract::State, Json};
use serde_json::json;
use crate::state::AppState;

pub async fn root_router(State(_state): State<AppState>) -> Json<serde_json::Value> {
    Json(json!({ "message": "Hello, World!" }))
}

//pub async fn root_router(State(state): State<AppState>) -> String {
//    format!("Hello, World! {}", get_time().await)
//
```

# routes/user_routes.rs

```rs
use axum::{
    extract::{State},
    http::StatusCode,
    routing::{post},
    Router,
    Json,
};
use crate::app_state::AppState;
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
```

# routes/mod.rs

```rs
pub mod user_routes;
pub mod root_routes;
```

# dtos/user_dtos.rs

```rs
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDTO {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCreateDTO {
    pub name: String,
}

```

# dtos/mod.rs

```rs
pub mod user_dtos;
```

# lib.rs

```rs
pub mod state;
pub mod dtos;
pub mod routes;
pub mod utils;
```

# main.rs

```rs
use std::sync::{Mutex, Arc};
use axum::{routing::get, Router};
use my_run::state::AppState;
use my_run::routes::root_routes::root_router;
use my_run::routes::user_routes::user_router;


#[tokio::main]
async fn main() {
    let state = AppState {
        db: Arc::new(Mutex::new(vec![])),
    };

    let app = Router::new()
        .route("/", get(root_router))
        .nest("/users", user_router())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}

```

# utils/datetime_utils.rs

```rs
use chrono::{DateTime, Local};

pub async fn get_time() -> String {
    let now: DateTime<Local> = Local::now();
    now.format("%Y-%m-%d %H:%M:%S,%3f").to_string()
}
```

# utils/mod.rs

```rs
pub mod datetime_utils;
```
