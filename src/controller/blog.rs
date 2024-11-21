use anyhow::Result;
use axum::{
    body::Body,
    extract::{Path, State},
    http::{header, Request, Response},
    response::IntoResponse,
    Extension, Json,
};
use rusqlite::Connection;
use tracing::{instrument, warn};

use crate::{
    error::ServerError,
    model::{Markdown, MarkdownMetadata, PostInfo, User},
    view, AppState,
};

#[instrument]
pub async fn load_markdown_content_api(
    State(state): State<AppState>,
    Path(postname): Path<String>,
) -> Result<impl IntoResponse, ServerError> {
    let connection: Connection = state.get_connection()?;
    Markdown::add_views_to_markdown(connection, &postname).await?;
    let markdown = Markdown::new(postname)?;
    Ok(Response::builder()
        .header(header::CONTENT_TYPE, "text/plain")
        .body(markdown.content)
        .unwrap())
}

#[instrument]
pub async fn markdown_list_api(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
) -> Result<impl IntoResponse, ServerError> {
    let sqlite_conn = state.get_connection()?;

    let markdownlist: Vec<(MarkdownMetadata, PostInfo)> =
        Markdown::list_private_markdown_info(sqlite_conn, user.id)
            .await?
            .into_iter()
            .filter(|i| i.0.finished)
            .collect();

    Ok(Json(markdownlist))
}

#[instrument]
pub async fn load_markdown_similar_posts_api(
    State(state): State<AppState>,
    Path(postname): Path<String>,
) -> Result<impl IntoResponse, ServerError> {
    let markdown = Markdown::new(postname)?;

    let similar_posts_metadata = markdown.metadata.similar_posts.iter().map(|i| {
        let connection: Connection = state.get_connection().unwrap();
        async move {
            {
                Markdown::list_markdown_info_of_post(connection, format!("{}.md", i))
                    .await
                    .map_err(|e| warn!("{e}"))
            }
        }
    });

    let similar_posts_metadata: Vec<(MarkdownMetadata, PostInfo)> =
        futures::future::join_all(similar_posts_metadata)
            .await
            .into_iter()
            .flatten()
            .collect();

    Ok(Json(similar_posts_metadata))
}

#[instrument]
pub async fn show(
    State(state): State<AppState>,
    Path(postname): Path<String>,
) -> Result<impl IntoResponse, ServerError> {
    let connection: Connection = state.get_connection()?;
    Markdown::add_views_to_markdown(connection, &postname).await?;

    let markdown = Markdown::new(postname)?;

    let similar_posts_metadata = markdown.metadata.similar_posts.iter().map(|i| {
        let connection: Connection = state.get_connection().unwrap();
        async move {
            {
                Markdown::list_markdown_info_of_post(connection, format!("{}.md", i))
                    .await
                    .map_err(|e| warn!("{e}"))
            }
        }
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

#[cfg(test)]
mod tests {
    use axum::body::to_bytes;

    use crate::MockAppState;

    use super::*;

    #[tokio::test]
    async fn it_should_return_plain_text_markdown() {
        let mock_app_state = AppState::new_mock("./data/blog.test.sqlite").unwrap();
        let state = State(mock_app_state);
        let path = Path("test-markdown".to_string());

        let content = load_markdown_content_api(state, path)
            .await
            .unwrap()
            .into_response()
            .into_body();
        let body_bytes = to_bytes(content, usize::MAX).await.unwrap();
        let content = String::from_utf8(body_bytes.to_vec()).unwrap();

        assert_eq!(
            content,
            "<h1>Test Markdown</h1>
<blockquote>
<p>test markdown</p>
</blockquote>
<blockquote>
<p>It will be published at <strong>SOON</strong></p>
</blockquote>\n"
        );
    }

    #[tokio::test]
    async fn it_should_return_an_array_of_similar_posts() {
        let mock_app_state = AppState::new_mock("./data/blog.test.sqlite").unwrap();
        let state = State(mock_app_state);
        let path = Path("test-markdown".to_string());

        let content = load_markdown_similar_posts_api(state, path)
            .await
            .unwrap()
            .into_response()
            .into_body();

        let body_bytes = to_bytes(content, usize::MAX).await.unwrap();
        let content: Vec<(MarkdownMetadata, PostInfo)> =
            serde_json::from_slice(&body_bytes).unwrap();

        assert_eq!(
            content,
            [(
                MarkdownMetadata {
                    filename: "test-markdown-2".to_string(),
                    title: "test-markdown-title".to_string(),
                    subtitle: "test-markdown-subtitle".to_string(),
                    description: "test-markdown-description".to_string(),
                    tags: vec!["rust".to_string(), "test".to_string()],
                    similar_posts: vec!["test-markdown".to_string()],
                    date: "2024-04-03t17:52:00".to_string(),
                    finished: false,
                    image_preview: None,
                    owner: None
                },
                PostInfo {
                    id: "test-markdown-2".to_string(),
                    views: 0
                }
            )]
        );
    }
}
