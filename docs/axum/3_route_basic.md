# State

`State` is a tuple struct

- State source code is
  ```rust
  pub struct State<S>(pub S);
  ```
- State is a wrapper of `Arc<AppState>`

  ```rust
  async fn root(State(state): State<Arc<AppState>>) -> &'static str {
    println!("{:?}", state);
    "Welcome!"
  }

  // ------------ equals to ------------

  async fn root(wrapper: State<Arc<AppState>>) -> &'static str {
      let state = wrapper.0;
      println!("{:?}", state);
      "Welcome!"
  }
  ```

# routes group

```rs
pub fn user_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(list_users).post(create_user))
        .route("/query", post(query_users))
        .route(
            "/{id}",
            get(get_user_by_id).put(update_user).delete(delete_user),
        )
}
```

# route method parameters

- query: `Query(query): Query<UserQueryDto>`
- path: `Path(id): Path<u64>`
- data: `Json(payload): Json<UserCreateDto>`
- state: `State(state): State<Arc<AppState>>`

```rs
// POST /users/:id
async fn xxx(
    State(state): State<Arc<AppState>>,
    Path(id): Path<u64>,
    Json(payload): Json<UserCreateDto>,
) -> (StatusCode, Json<UserDto>) {
    let encrypted_password = format!("hashed-{}", payload.password);

    let user = UserDto {
        id: state.next_id.fetch_add(1, Ordering::Relaxed) as u64,
        username: payload.username,
        password: encrypted_password,
    };

    state.db.lock().unwrap().push(user.clone());

    if ... {
        Ok(Json(user.clone()))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
    (StatusCode::CREATED, Json(user))
}
```

# structure

```
├── Cargo.toml
└── src
    ├── main.rs
    ├── dtos
    │   ├── auth_dtos.rs
    │   ├── mod.rs
    │   └── user_dtos.rs
    └── routes
        ├── auth.rs
        ├── mod.rs
        └── user.rs
```

# main.rs

```rs
use axum::{extract::State,routing::get, Router};
use std::sync::{atomic::AtomicUsize, Arc, Mutex};
use dtos::user_dtos::UserDto;

mod dtos;
mod routes;

// 1. define AppState to store state shared in multiple routes/threading context
#[derive(Debug)]
pub struct AppState {
    db: Mutex<Vec<UserDto>>,  // rw vec
    next_id: AtomicUsize,
}

#[tokio::main]
async fn main() {
    // 2. create AppState
    let app_state = Arc::new(AppState {
        db: Mutex::new(vec![]),
        next_id: AtomicUsize::new(1),
    });

    // 3. create router with state
    let app = Router::new()
        .route("/", get(root))
        // define routes
        .nest("/users", routes::user_routes::user_routes())
        .nest("/auth", routes::auth_routes::auth_routes())
        // pass in state
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .await
        .unwrap();
}

// 4. define root route
async fn root(State(state): State<Arc<AppState>>) -> &'static str {
    println!("{:?}", state);
    "Welcome!"
}
```

# routes/mod.rs

```rs
pub mod auth_routes;
pub mod user_routes;
```

# routes/user_routes.rs

```rs
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

// 1. define routes
pub fn user_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(list_users).post(create_user))
        .route("/query", post(query_users))
        .route(
            "/{id}",
            get(get_user_by_id).put(update_user).delete(delete_user),
        )
}

// POST /users
async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UserCreateDto>,
) -> (StatusCode, Json<UserDto>) {
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
    let mut db = state.db.lock().unwrap();
    if let Some(user) = db.iter_mut().find(|u| u.id == id) {
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
```
