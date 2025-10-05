use axum::{routing::get, Router};
use dotenvy::dotenv;
use std::{net::SocketAddr, sync::{Arc, Mutex}};
use tokio;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::info;
use eng_be::{telemetry, api_docs, routes, AppState};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() {
    // 1. Initialize Telemetry (Tracing & Logging)
    telemetry::init_tracing();

    // 2. Load environment variables
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    info!("Hello, world! {}", database_url);



    // 3. Create Application State
    let app_state = Arc::new(AppState {
        db: Mutex::new(Vec::new()),
    });

    // 5. Build the main router
    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api_docs::ApiDoc::openapi()))
        .route("/", get(|| async { "Matt API Running" }))
        // Combine routes from different modules
        .nest("/users", routes::user::user_routes())
        // Add middleware for logging and CORS
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        // Inject application state
        .with_state(app_state);

    // 6. Start the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .expect("Server failed");
}