# Cargo.toml

```toml
[dependencies]
# Web Framework
axum = { version = "0.8.5", features = ["macros"] }
tokio = { version = "1.47.1", features = ["full"] }
```

# main.rs

```rs
use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/",  get(root));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}
```

# start & access

```
cargo run
curl http://localhost:3000
```
