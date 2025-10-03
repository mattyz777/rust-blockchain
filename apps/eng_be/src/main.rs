use axum::{routing::get, Router};
use dotenvy::dotenv;
use std::net::SocketAddr;
use tokio;

#[tokio::main]
async fn main() {
    dotenv().ok(); 
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    println!("Hello, world! {}", database_url);

    // 构建路由：GET / -> 返回字符串
    let app = Router::new().route("/", get(|| async { "Hello, World! 🚀" }));

    // 绑定地址
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🔥 Listening on {}", addr);

    // 使用 tokio::net::TcpListener 来绑定端口
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind address");

    axum::serve(listener, app)
        .await
        .expect("Server failed");
}
