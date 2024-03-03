use axum::{response::IntoResponse, routing::get, Router};

use crate::{
    error::{Result, ResultPath},
    model::Markdown,
    view::{self, root as rootView},
};

pub async fn root_page() -> Result<impl IntoResponse> {
    rootView()
}

#[derive(Default)]
pub struct Blog {
    pub routes: axum::Router,
}

impl Blog {
    pub fn new() -> Self {
        Self {
            routes: Router::new()
                .route("/", get(Self::index))
                .route("/:id", get(Self::show)),
        }
    }

    async fn index() -> Result<impl IntoResponse> {
        // Get all blogpost names
        // extract all MarkdownMetadata
        // Append each metadata to a Vec
        // Return a htmx of all list for the entry page
        Ok(view::blog::index())
    }

    async fn show(ResultPath(postname): ResultPath<String>) -> Result<impl IntoResponse> {
        let markdown = Markdown::new(postname)?;
        Ok(view::blog::show(markdown))
    }
}
