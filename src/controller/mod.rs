use anyhow::Result;
use axum::{extract::State, response::IntoResponse};
use tracing::instrument;

pub mod blog;

use crate::{
    config::blog::Blog,
    error::ServerError,
    model::{Markdown, MarkdownMetadata, PostInfo},
    view::homepage,
    AppState,
};

#[instrument]
pub async fn home(State(state): State<AppState>) -> Result<impl IntoResponse, ServerError> {
    let sqlite_conn = state.env_state.get_connection()?;
    let markdownlist: Vec<(MarkdownMetadata, PostInfo)> = Markdown::list_private_markdown_info(
        sqlite_conn,
        Blog {
            path: state.config_state.blog_config.application,
            user: "application_main_user".to_string(),
        },
    )
    .await?
    .into_iter()
    .filter(|i| i.0.finished)
    .collect();

    homepage(markdownlist)
}
