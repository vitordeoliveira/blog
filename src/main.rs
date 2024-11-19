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
    let sqlite_db = env!("SQLITE_DB");
    let assets_path = env!("CARGO_MANIFEST_DIR");

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

    let app_state: AppState = AppState::new(sqlite_db, &rust_env)?;
    let app = app::new_app(app_state, assets_path).await?;

    tracing::info!("router initialized, now listening on port {}", port);

    axum::serve(listener, app)
        .await
        .context("failed to serve server")
        .unwrap();

    Ok(())
}
