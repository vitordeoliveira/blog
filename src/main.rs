use anyhow::{Context, Result};
use blog::{app, config, error::ServerError, AppState, ConfigState, EnvState};
use std::env;

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

    let blog_config_path = env::var("BLOG_CONFIG_PATH").unwrap_or("./blog.config.toml".to_string());

    config::tracing::Tracing::setup(&tracer_url, rust_log)?;

    let listener = tokio::net::TcpListener::bind(format!("{host}:{port}"))
        .await
        .context("Failed to start tokio listener")
        .unwrap();

    let config_state: ConfigState = ConfigState::new(&blog_config_path, assets_path);
    let env_state: EnvState = EnvState::new(&sqlite_db_path, &rust_env)?;
    let state = AppState {
        config_state,
        env_state,
    };

    let app = app::new_app(state).await?;

    tracing::info!("router initialized, now listening on port {}", port);

    axum::serve(listener, app)
        .await
        .context("failed to serve server")
        .unwrap();

    Ok(())
}
