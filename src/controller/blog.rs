use axum::{extract::Path, response::IntoResponse, routing::get, Router};

use crate::{error::Result, model::Markdown, view};

#[derive(Default)]
pub struct Blog {
    pub routes: axum::Router,
}

impl Blog {
    pub fn new() -> Self {
        Self {
            routes: Router::new().route("/:id", get(Self::show)),
        }
    }

    async fn show(Path(postname): Path<String>) -> Result<impl IntoResponse> {
        let markdown = Markdown::new(postname)?;
        Ok(view::blog::show(markdown))
    }
}
