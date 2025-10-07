# AppState

`AppState` is used to store global shared data that can be safely accessed across multiple threads.

- Database connection pool
- Configuration
- Cache client (e.g. Redis)
- Logic/service instances
- Other shared runtime data

# AppState rules

- AppState struct must implement `Clone` trait
- usually use `Arc<T>` or `Arc<Mutext<T>>` to wrap the data
- init AppState in main function and pass to router
- use `State<T>` to extract AppState in handler

# AppState example

```toml
[dependencies]
# Web Framework
axum = { version = "0.8.5", features = ["macros"] }
tokio = { version = "1.47.1", features = ["full"] }
```

```rs
use axum::{extract::State, routing::get, Router};
use std::sync::Arc;

// 1. Define AppState and must implement Clone trait
#[derive(Clone)]
struct AppState {
    db: String, // e.g. SqlitePool
    app_name: String,
}

#[tokio::main]
async fn main() {
    // 2. Initialize AppState
    let state = Arc::new(AppState {
        db: "db pool".to_string(),
        app_name: "API".to_string(),
    });

    // 3. Build app with state
    let app = Router::new()
        .route("/", get(root_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}

// 4. Define a handler that accepts AppState
async fn root_handler(State(state): State<Arc<AppState>>) -> String {
    format!("Welcome to {} - {}!", state.app_name, state.db)
}
```
