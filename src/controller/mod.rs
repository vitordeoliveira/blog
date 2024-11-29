use anyhow::Result;
use axum::{extract::State, response::IntoResponse};
use tracing::instrument;

pub mod blog;

use crate::{
    error::ServerError,
    model::{Markdown, MarkdownMetadata, PostInfo},
    view::homepage,
    EnvState,
};

#[instrument]
pub async fn home(State(state): State<EnvState>) -> Result<impl IntoResponse, ServerError> {
    let sqlite_conn = state.get_connection()?;
    let markdownlist: Vec<(MarkdownMetadata, PostInfo)> = Markdown::list_markdown_info(sqlite_conn)
        .await?
        .into_iter()
        .filter(|i| i.0.finished)
        .collect();

    homepage(markdownlist)
}
