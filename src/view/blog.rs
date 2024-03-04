use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

use crate::{
    error::{Error, Result},
    model::{Markdown, MarkdownMetadata},
};

#[derive(Template)]
#[template(path = "blog.html")]
struct BlogTemplate {
    metadata: MarkdownMetadata,
    content: String,
}

pub fn show(Markdown { metadata, content }: Markdown) -> Result<impl IntoResponse> {
    let root = BlogTemplate { metadata, content };

    let html = match root.render() {
        Ok(html) => html,
        Err(_) => return Err(Error::InternalServer("Error on render".to_string())),
    };

    Ok((StatusCode::OK, Html(html)))
}
