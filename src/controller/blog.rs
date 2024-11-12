use anyhow::Result;
use axum::{extract::Path, response::IntoResponse};
use tracing::{info, warn};

use crate::{
    error::ServerError,
    model::{Markdown, MarkdownMetadata, PostInfo},
    view,
};

pub async fn show(Path(postname): Path<String>) -> Result<impl IntoResponse, ServerError> {
    info!("show");
    Markdown::add_views_to_markdown(&postname).await?;
    let markdown = Markdown::new(postname)?;

    let similar_posts_metadata = markdown.metadata.similar_posts.iter().map(|i| async move {
        Markdown::list_markdown_info_of_post(format!("{}.md", i))
            .await
            .map_err(|e| warn!("{e}"))
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
