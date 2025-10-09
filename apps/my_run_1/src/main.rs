use std::sync::{Mutex, Arc};
use axum::{routing::get, Router};
use my_run_1::app_state::AppState;
use my_run_1::routes::root_routes::root_router;
use my_run_1::routes::user_routes::user_router;


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
