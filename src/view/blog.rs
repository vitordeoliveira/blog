use askama::Template;
use axum::{
    extract::{rejection::PathRejection, Path},
    http::StatusCode,
    response::{Html, IntoResponse},
};
use axum_macros::debug_handler;

use crate::error::{self, Error, Result};

#[derive(Template)]
#[template(path = "blog.html")]
struct BlogTemplate {
    title: String,
    id: u16,
}

#[debug_handler]
pub async fn blog_view(
    blog_id: std::result::Result<Path<u16>, PathRejection>,
) -> Result<impl IntoResponse> {
    let test = match blog_id {
        Ok(e) => e,
        Err(_) => return Err(Error::InternalServerError),
    };
    let root = BlogTemplate {
        title: "vitor.ws".to_string(),
        id: test.0,
    };

    let html = match root.render() {
        Ok(html) => html,
        Err(_) => return Err(error::Error::InternalServerError),
    };

    Ok((StatusCode::OK, Html(html)))
}
