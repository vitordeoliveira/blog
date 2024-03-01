use askama::Template;
use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse},
};
use axum_macros::debug_handler;

use crate::error::{self, Result, ResultPath};

#[derive(Template)]
#[template(path = "blog.html")]
struct BlogTemplate {
    title: String,
    id: u16,
}

#[debug_handler]
pub async fn blog_view(ResultPath(blog_id): ResultPath<u16>) -> Result<impl IntoResponse> {
    // pub async fn blog_view(blog_id: Result<Path<u16>>) -> Result<impl IntoResponse> {
    let root = BlogTemplate {
        title: "vitor.ws".to_string(),
        id: blog_id,
    };

    let html = match root.render() {
        Ok(html) => html,
        Err(_) => return Err(error::Error::InternalServerError),
    };

    Ok((StatusCode::OK, Html(html)))
}
