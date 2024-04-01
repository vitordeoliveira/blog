use axum::{extract::Path, response::IntoResponse, routing::get, Router};
use tracing::{info, warn};

use crate::{
    error::Result,
    model::{Markdown, MarkdownMetadata, PostInfo},
    view,
};

#[derive(Default)]
pub struct Blog {
    pub routes: axum::Router,
    // pub routes_api: axum::Router,
}

impl Blog {
    pub fn new() -> Self {
        Self {
            routes: Router::new().route("/:id", get(Self::show)),
        }
    }

    async fn show(Path(postname): Path<String>) -> Result<impl IntoResponse> {
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
}
