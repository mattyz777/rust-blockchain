# structure

```
eng_be/
├── .env                  # Environment variables (e.g., Supabase URL, API keys)
├── Cargo.toml            # Rust project dependencies and metadata
├── src/
│   ├── main.rs
```

# .env

```properties
DATABASE_URL=aaa
```

# Cargo.toml

```toml
[dependencies]
dotenvy = "0.15.0"
```

# test dotenv

```rs
use dotenvy::dotenv;
use std::env::var;

fn main() {
    dotenv().ok(); // load .env file
    let database_url = var("DATABASE_URL").expect("DATABASE_URL must be set");

    println!("Hello, world! {}", database_url);
}

```
