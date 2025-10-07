use axum::{routing::get, Router};
use std::sync::{atomic::AtomicUsize, Arc, Mutex};
use dtos::user_dtos::UserDto;

mod dtos;
mod routes;

pub struct AppState {
    db: Mutex<Vec<UserDto>>,
    next_id: AtomicUsize,
}

#[tokio::main]
async fn main() {
    let app_state = Arc::new(AppState {
        db: Mutex::new(vec![]),
        next_id: AtomicUsize::new(1),
    });

    let app = Router::new()
        .route("/", get(root))
        .nest("/users", routes::user_routes::user_routes())
        .nest("/auth", routes::auth_routes::auth_routes())
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 Server listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Welcome to the `my_run` API!"
}