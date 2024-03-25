use axum::{
    extract::Path,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use tracing::info;

use crate::{error::Result, model::Markdown, view};

#[derive(Default)]
pub struct Blog {
    pub routes: axum::Router,
    // pub routes_api: axum::Router,
}

impl Blog {
    pub fn new() -> Self {
        Self {
            routes: Router::new().route("/:id", get(Self::show)),
            // routes_api: Router::new().route("/:postname", post(Self::add_view)),
        }
    }

    async fn show(Path(postname): Path<String>) -> Result<impl IntoResponse> {
        info!("show");
        Markdown::add_views_to_markdown(&postname).await?;
        let markdown = Markdown::new(postname)?;
        Ok(view::blog::show(markdown))
    }

    async fn add_view(Path(postname): Path<String>) -> Result<impl IntoResponse> {
        info!("add_view");
        // Markdown::add_views_to_markdown(&postname).await?;
        Ok(())
    }
}
