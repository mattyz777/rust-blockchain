use std::sync::Arc;
use std::net::SocketAddr;

use axum::{
    routing::get,
    Router,
    middleware::Next,
    http::Request,
    response::{Response}, 
    body::Body,
};
use dotenvy::dotenv;
use sea_orm::Database;
use my_run::state::AppState; 
use my_run::routes::{root_routes::root_router, user_routes::user_router};
use tokio::time::Instant;

/// Simple logging middleware
async fn log_requests(req: Request<Body>, next: Next) -> Response {
    let method = req.method().clone();
    let path = req.uri().path().to_string();
    let start = Instant::now();

    let response = next.run(req).await;

    let duration = start.elapsed();
    println!("{} {} -> {} ({}ms)", method, path, response.status(), duration.as_millis());

    response
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")?;
    let db = Arc::new(Database::connect(&database_url).await?);
    println!("✅ Database connected successfully: {}", database_url);

    let state = AppState { db }; 

    let app = Router::new()
        .route("/api", get(root_router))
            .nest("/users", user_router())
        .with_state(state)
        .layer(axum::middleware::from_fn(log_requests));

    let addr: SocketAddr = "0.0.0.0:3000".parse()?;
    
    let listener = tokio::net::TcpListener::bind(addr).await?;

    println!("🚀 Server listening on http://{}", addr);
    println!("📄 Swagger UI: http://{}/swagger-ui", addr);
    println!("📑 OpenAPI JSON: http://{}/api-doc/openapi.json", addr);

    axum::serve(listener, app).await?; 

    Ok(())
}