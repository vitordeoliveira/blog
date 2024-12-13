use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::{header, Response},
    response::IntoResponse,
    Extension, Json,
};
use rusqlite::Connection;
use tracing::{info, instrument, warn};

use crate::{
    config::blog::Blog,
    error::ServerError,
    model::{Markdown, MarkdownMetadata, PostInfo, User},
    view, AppState, EnvState,
};

#[instrument]
pub async fn load_markdown_content_api(
    State(state): State<EnvState>,
    Path(postname): Path<String>,
    Extension(blog_config): Extension<Blog>,
) -> Result<impl IntoResponse, ServerError> {
    let connection: Connection = state.get_connection()?;
    let markdown = Markdown::new(format!("{}/{}.md", blog_config.path, postname.clone()))?;
    Markdown::add_views_to_markdown(connection, &postname).await?;
    Ok(Response::builder()
        .header(header::CONTENT_TYPE, "text/plain")
        .header("X-Content-Title", markdown.metadata.title)
        .header("X-Content-Description", markdown.metadata.description)
        .header(
            "X-Content-Author",
            markdown.metadata.author.unwrap_or_default(),
        )
        .header(
            "X-Content-Image-Preview",
            markdown.metadata.image_preview.unwrap_or_default(),
        )
        .header(
            "X-Content-Tags",
            markdown.metadata.tags.unwrap_or_default().join(","),
        )
        .body(markdown.content)
        .unwrap())
}

#[instrument]
pub async fn markdown_list_api(
    State(state): State<EnvState>,
    Extension(user): Extension<User>,
    Extension(blog_config): Extension<Blog>,
) -> Result<impl IntoResponse, ServerError> {
    info!("Listing markdown list of user: {}", user.id);
    let sqlite_conn = state.get_connection()?;
    let markdownlist: Vec<(MarkdownMetadata, PostInfo)> =
        Markdown::list_private_markdown_info(sqlite_conn, blog_config).await?;

    Ok(Json(markdownlist))
}

#[instrument]
pub async fn load_markdown_similar_posts_api(
    State(state): State<EnvState>,
    Path(postname): Path<String>,
    Extension(blog_config): Extension<Blog>,
) -> Result<impl IntoResponse, ServerError> {
    let markdown = Markdown::new(postname)?;

    let similar_posts_metadata = markdown.metadata.similar_posts.iter().flatten().map(|i| {
        let connection: Connection = state.get_connection().unwrap();
        let md_folder_path = blog_config.path.clone();
        async move {
            {
                Markdown::list_markdown_info_of_a_post(
                    connection,
                    format!("{}/{}.md", md_folder_path, i),
                )
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
    let connection: Connection = state.env_state.get_connection()?;
    Markdown::add_views_to_markdown(connection, &postname).await?;

    let post_full_name = format!(
        "{}/{}.md",
        state.config_state.blog_config.application, postname
    );
    let markdown = Markdown::new(post_full_name)?;

    let similar_posts_metadata = markdown.metadata.similar_posts.iter().flatten().map(|i| {
        let connection: Connection = state.env_state.get_connection().unwrap();
        let md_folder_path = state.config_state.blog_config.application.clone();
        async move {
            {
                Markdown::list_markdown_info_of_a_post(
                    connection,
                    format!("{}/{}.md", md_folder_path, i),
                )
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

    use crate::{mock::MockAppState, model::mock::MockUser};

    use super::*;

    #[tokio::test]
    async fn it_should_return_empty_when_invalid_metadata() {
        let mock_app_state = EnvState::new_mock("./data/blog.test.sqlite").unwrap();
        let state = State(mock_app_state);
        let user = User::new_mock().unwrap();
        let extension = Extension(user);

        let blog_config = Blog {
            path: "mock_path".to_string(),
            user: "mockuser".to_string(),
        };

        let extension_blog_config = Extension(blog_config);

        let content = markdown_list_api(state, extension, extension_blog_config)
            .await
            .unwrap()
            .into_response()
            .into_body();

        let body_bytes = to_bytes(content, usize::MAX).await.unwrap();
        let content: Vec<(MarkdownMetadata, PostInfo)> =
            serde_json::from_slice(&body_bytes).unwrap();

        assert_eq!(content, [])
    }

    #[tokio::test]
    async fn it_should_return_plain_text_markdown() {
        let mock_app_state = EnvState::new_mock("./data/blog.test.sqlite").unwrap();
        let state = State(mock_app_state);
        let path = Path("test-markdown".to_string());

        let blog_config = Blog {
            path: "mock_path".to_string(),
            user: "mockuser".to_string(),
        };

        let extension_blog_config = Extension(blog_config);
        let content = load_markdown_content_api(state, path, extension_blog_config)
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
        let mock_app_state = EnvState::new_mock("./data/blog.test.sqlite").unwrap();
        let state = State(mock_app_state);
        let path = Path("test-markdown".to_string());

        let blog_config = Blog {
            path: "mock_path".to_string(),
            user: "mockuser".to_string(),
        };

        let extension_blog_config = Extension(blog_config);
        let content = load_markdown_similar_posts_api(state, path, extension_blog_config)
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
                    version: None,
                    filename: "test-markdown-2".to_string(),
                    title: "test-markdown-title".to_string(),
                    subtitle: "test-markdown-subtitle".to_string(),
                    description: "test-markdown-description".to_string(),
                    tags: Some(vec!["rust".to_string(), "test".to_string()]),
                    similar_posts: Some(vec!["test-markdown".to_string()]),
                    date: Some("2024-04-03t17:52:00".to_string()),
                    finished: false,
                    ..Default::default()
                },
                PostInfo {
                    id: "test-markdown-2".to_string(),
                    views: 0
                }
            )]
        );
    }
}
