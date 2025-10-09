# Cargo.toml

```toml
[dependencies]
# Web Framework
axum = { version = "0.8.5", features = ["macros"] }
tokio = { version = "1.47.1", features = ["full"] }

chrono = { version = "0.4.42", features = ["serde"] }
```

# main.rs

```rs
use axum::{routing::get, Router};
use chrono::{Local, DateTime};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/",  get(root));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}

async fn get_time () -> String {
    let now: DateTime<Local> = Local::now();
    now.format("%Y-%m-%d %H:%M:%S,%3f").to_string()
}

async fn root() -> String {
    format!("Hello, World! {}", get_time().await)
}
```

# start & access

```
cargo run
curl http://localhost:3000
```
