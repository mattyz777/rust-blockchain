# src/db.rs

```rs
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;
use dotenvy::var;

pub async fn connect_db() -> Result<PgPool, sqlx::Error> {
    let database_url = var("DATABASE_URL").expect("DATABASE_URL must be set in .env");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&database_url)
        .await?;

    Ok(pool)
}
```
