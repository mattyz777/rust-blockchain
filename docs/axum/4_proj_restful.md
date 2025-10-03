# target

- load env
- create router
- add twoer trace/cors middleware
- start server

# build router

main.rs

```rs
use axum::{routing::get, Router};
use dotenvy::dotenv;
use std::net::SocketAddr;
use tokio;
use tower_http::trace::TraceLayer;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    // 1. Load environment variables
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Hello, world! {}", database_url);

    // 2. Initialize Tracing (Logging)
    tracing_subscriber::fmt::init();

    // 5. Build the main router
    let app = Router::new()
        .route("/", get(|| async { "Matt API Running" }))
        // Combine routes from different modules
        .nest("/auth", routes::user::auth_routes())
        .nest("/users", routes::user::user_routes())
        // Add middleware for logging and CORS
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        // Inject application state
        .with_state(app_state);

    // 6. Start the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind address");

    axum::serve(listener, app)
        .await
        .expect("Server failed");
}


```
