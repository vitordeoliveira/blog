use std::env;

use anyhow::{Context, Result};
use blog::{
    app,
    config::{self},
    error::ServerError,
    AppState,
};

#[tokio::main]
async fn main() -> Result<(), ServerError> {
    let host = env!("SERVER_HOST");
    let port = env!("SERVER_PORT");
    let rust_log = env!("RUST_LOG");
    let assets_path = env!("CARGO_MANIFEST_DIR");

    let sqlite_db_path = env::var("SQLITE_DB_PATH")
        .context("SQLITE_DB must be defined")
        .unwrap();
    let rust_env = env::var("RUST_ENV")
        .context("RUST_ENV must be defined")
        .unwrap();
    let tracer_url = env::var("TRACER_URL")
        .context(" TRACER_URL must be defined")
        .unwrap();

    config::tracing::Tracing::setup(&tracer_url, rust_log)?;

    let listener = tokio::net::TcpListener::bind(format!("{host}:{port}"))
        .await
        .context("Failed to start tokio listener")
        .unwrap();

    let app_state: AppState = AppState::new(&sqlite_db_path, &rust_env)?;
    let app = app::new_app(app_state, assets_path).await?;

    tracing::info!("router initialized, now listening on port {}", port);

    axum::serve(listener, app)
        .await
        .context("failed to serve server")
        .unwrap();

    Ok(())
}
