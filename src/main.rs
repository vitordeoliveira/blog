use anyhow::{Context, Result};
use blog::{app, config::Config};

#[tokio::main]
async fn main() -> Result<()> {
    let host = env!("SERVER_HOST");
    let port = env!("SERVER_PORT");
    let rust_log = env!("RUST_LOG");
    let sqlite_db = env!("SQLITE_DB");
    let assets_path = env!("CARGO_MANIFEST_DIR");

    Config::logging(rust_log).await;

    let listener = tokio::net::TcpListener::bind(format!("{host}:{port}"))
        .await
        .context("Failed to start tokio listener")
        .unwrap();

    let app = app::new_app(sqlite_db, assets_path).await?;

    tracing::info!("router initialized, now listening on port {}", port);

    axum::serve(listener, app)
        .await
        .context("failed to serve server")
        .unwrap();

    Ok(())
}
