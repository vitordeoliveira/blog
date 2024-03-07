use axum::response::IntoResponse;

pub mod blog;

use crate::{error::Result, model::Markdown, view::homepage};

pub async fn home() -> Result<impl IntoResponse> {
    let markdownlist = Markdown::list()?
        .into_iter()
        .filter(|i| i.finished)
        .collect();
    homepage(markdownlist)
}
