use axum::{routing::get, Router};
use dotenvy::dotenv;
use std::{net::SocketAddr, sync::{Arc, Mutex}};
use tokio;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::dtos::user_dto::{CreateUserRequest, UpdateUserRequest}; 
use crate::models::user_model::User; 

pub mod error;
pub mod models;
pub mod dtos;
pub mod routes;

pub struct AppState {
    db: Mutex<Vec<User>>,
}

#[derive(OpenApi)]
#[openapi(
    paths(
        routes::user::create_user,
        routes::user::list_users,
        routes::user::get_user,
        routes::user::update_user,
        routes::user::delete_user,
    ),
    components(schemas(User, CreateUserRequest, UpdateUserRequest)),
    tags((name = "User API", description = "User management endpoints"))
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    // 1. Initialize Tracing with advanced configuration
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "eng_be=info,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

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
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/", get(|| async { "Matt API Running" }))
        // Combine routes from different modules
        .nest("/users", crate::routes::user::user_routes())
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