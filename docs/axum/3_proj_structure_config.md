# folder structure

```
eng_be/
├── .env                  # Environment variables (e.g., Supabase URL, API keys)
├── Cargo.toml            # Rust project dependencies and metadata
├── migrations/           # Database migration files (managed by sqlx-cli)
│   ├── 0001_create_users_table.sql
│   └── ...
├── src/
│   ├── main.rs           # Application entry point, server setup, and router composition
│   ├── error.rs          # Custom error types and response handlers
│   ├── db.rs             # Database connection pooling and initialization
│   ├── lib.rs            # For shared types, utilities, and re-exporting modules
│   ├── models/           # Database struct representations (structs mapping to tables)
│   │   ├── user.rs
│   │   └── mod.rs
│   ├── handlers/         # Core business logic and HTTP handler functions
│   │   ├── user.rs       # CRUD functions for the user module
│   │   ├── mod.rs
│   │   └── product.rs    # (Example of another module)
│   ├── routes/           # Defines specific API routes and combines handlers
│   │   ├── user.rs       # User routes: /users, /users/:id
│   │   ├── auth.rs       # Auth routes: /signup, /login
│   │   ├── mod.rs
│   │   └── product.rs    # (Example of another module)
│   ├── middleware/       # Custom Axum middleware (like the "interceptor")
│   │   ├── auth.rs       # Authentication middleware (e.g., JWT validation)
│   │   └── mod.rs
│   └── state.rs          # Application state (e.g., database pool, config) passed to handlers
└── tests/
    ├── integration_tests.rs
    └── ...
```

# init

```shell
cargo new eng_be
cd eng_be
```

# Cargo.toml

```toml
[package]
name = "eng_be"
version = "0.1.0"
edition = "2024"

[dependencies]
# Web Framework
axum = { version = "0.8.6", features = ["macros"] }
tokio = { version = "1", features = ["full"] }
tower-http = { version = "0.6.6", features = ["cors", "trace"] }

# Data & Config
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
dotenvy = "0.15"
uuid = { version = "1.8", features = ["serde", "v4"] }
chrono = { version = "0.4", features = ["serde"] }

# Database
sqlx = { version = "0.8.6", features = ["runtime-tokio-rustls", "postgres", "uuid", "chrono"] }

# Auth & Security
jsonwebtoken = { version = "10.0.0", features = ["rust_crypto"] }
bcrypt = "0.17.1"

```

# .env

```properties
DATABASE_URL="postgres://postgres:YOUR_PASSWORD@localhost:5432/postgres"
JWT_SECRET="YOUR_SUPER_SECURE_JWT_SECRET_KEY_HERE"
```
