use std::time::Duration;

use anyhow::{Context, Result};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

#[derive(Clone, Debug)]
pub struct AppState {
    pub sqlite_pool: SqlitePool,
}

impl AppState {
    pub async fn new(db_connection_str: &str) -> Result<Self> {
        let sqlite_pool = SqlitePoolOptions::new()
            .max_connections(25)
            .acquire_timeout(Duration::from_secs(3))
            .connect(db_connection_str)
            .await
            .context("sqlite database pool creation error")
            .expect("can't connect to database");

        // sqlx::migrate!("db/migrations").run(&sqlite_pool).await?;
        Ok(Self { sqlite_pool })
    }
}
