use anyhow::Result;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use rusqlite::Connection;
use tracing::{info, instrument, warn};

use crate::{
    error::ServerError,
    model::{Markdown, MarkdownMetadata, PostInfo},
    view, AppState,
};

#[instrument]
pub async fn show(
    State(state): State<AppState>,
    Path(postname): Path<String>,
) -> Result<impl IntoResponse, ServerError> {
    let connection: Connection = state.get_connection()?;
    Markdown::add_views_to_markdown(connection, &postname).await?;

    let markdown = Markdown::new(postname)?;

    let similar_posts_metadata = markdown.metadata.similar_posts.iter().map(|i| {
        let connection: Connection = state.get_connection().unwrap();
        async move {
            {
                Markdown::list_markdown_info_of_post(connection, format!("{}.md", i))
                    .await
                    .map_err(|e| warn!("{e}"))
            }
        }
    });

    let similar_posts_metadata: Vec<(MarkdownMetadata, PostInfo)> =
        futures::future::join_all(similar_posts_metadata)
            .await
            .into_iter()
            .flatten()
            // .filter_map(|e| e.ok())
            .collect();

    Ok(view::blog::show(markdown, similar_posts_metadata))
}
