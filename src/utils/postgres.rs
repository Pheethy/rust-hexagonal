use anyhow::Result;
use sqlx::postgres::{PgPool, PgPoolOptions};

pub async fn database_connect(database_url: &str) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    Ok(pool)
}
