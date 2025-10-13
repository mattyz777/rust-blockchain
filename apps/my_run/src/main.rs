use std::sync::Arc;
use axum::{routing::get, Router};
use dotenvy::dotenv;
use sea_orm::Database;
use std::env::var;
use my_run::state::AppState;
use my_run::routes::root_routes::root_router;
use my_run::routes::user_routes::user_router;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let database_url = var("DATABASE_URL").expect("DATABASE_URL must be set in .env");

    let state = AppState {
        db: Arc::new(Database::connect(database_url).await?),
    };

    let app = Router::new()
        .route("/", get(root_router))
        .nest("/users", user_router())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
    Ok(())
}