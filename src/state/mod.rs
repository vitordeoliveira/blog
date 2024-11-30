use anyhow::{Context, Result};
use axum::extract::FromRef;
use rusqlite::Connection;

use crate::{
    config::{self, blog::BlogConfig},
    error::ServerError,
};
refinery::embed_migrations!("migrations");

#[cfg(test)]
pub mod mock;

#[derive(Clone, Debug)]
pub struct AppState {
    pub config_state: ConfigState,
    pub env_state: EnvState,
}

impl FromRef<AppState> for ConfigState {
    fn from_ref(app_state: &AppState) -> ConfigState {
        app_state.config_state.clone()
    }
}

impl FromRef<AppState> for EnvState {
    fn from_ref(app_state: &AppState) -> EnvState {
        app_state.env_state.clone()
    }
}

#[derive(Clone, Debug)]
pub struct ConfigState {
    pub blog_config: BlogConfig,
    pub assets_path: String,
}

impl ConfigState {
    pub fn new(blog_config_path: &str, assets_path: &str) -> Self {
        Self {
            blog_config: config::blog::BlogConfig::from_file(blog_config_path),
            assets_path: assets_path.to_string(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct EnvState {
    pub sqlite_path: String,
    pub rust_env: String,
}

impl EnvState {
    pub fn new(sqlite_path: &str, rust_env: &str) -> Result<Self, ServerError> {
        let mut sqlite_conn = Connection::open(sqlite_path).map_err(ServerError::DBError)?;

        migrations::runner()
            .run(&mut sqlite_conn)
            .context("migration error")
            .unwrap();

        Ok(Self {
            sqlite_path: sqlite_path.to_string(),
            rust_env: rust_env.to_string(),
        })
    }

    pub fn get_connection(&self) -> Result<Connection, ServerError> {
        Connection::open(&self.sqlite_path).map_err(ServerError::DBError)
    }
}
