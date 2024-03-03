use std::fs;

use axum::{response::IntoResponse, routing::get, Router};

use crate::{
    error::{Error, Result, ResultPath},
    model::MarkdownMetadata,
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
        let file = format!("./blogpost/{}.md", &postname);
        let markdown_input =
            fs::read_to_string(file).map_err(|_| Error::PageNotFound(postname.clone()))?;
        let metadata = MarkdownMetadata::new(&markdown_input)?;
        let mut html_output = String::new();
        let res = MarkdownMetadata::extract(&markdown_input)?;
        let parser = pulldown_cmark::Parser::new(&res);
        pulldown_cmark::html::push_html(&mut html_output, parser);

        Ok(view::blog::show(html_output, metadata))
    }
}
