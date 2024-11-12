use anyhow::Result;
use axum::response::IntoResponse;

pub mod blog;

use crate::{
    error::ServerError,
    model::{Markdown, MarkdownMetadata, PostInfo},
    view::homepage,
};

pub async fn home() -> Result<impl IntoResponse, ServerError> {
    let markdownlist: Vec<(MarkdownMetadata, PostInfo)> = Markdown::list_markdown_info()
        .await?
        .into_iter()
        .filter(|i| i.0.finished)
        .collect();

    homepage(markdownlist)
}
