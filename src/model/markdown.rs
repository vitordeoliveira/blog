use std::fs;

use anyhow::Result;
use pulldown_cmark::Options;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use tracing::{info, instrument, warn};
use uuid::Uuid;
use yaml_front_matter::YamlFrontMatter;

use crate::{config::blog::Blog, error::ServerError};

use super::sqlite::{PostInfo, SqliteOperations};

pub struct Markdown {
    pub metadata: MarkdownMetadata,
    pub content: String,
}

impl Markdown {
    #[instrument]
    // TODO: TEST
    pub fn new(post_path: String) -> Result<Self, ServerError> {
        let file = post_path.to_string();
        let markdown_file =
            fs::read_to_string(file).map_err(|_| ServerError::PageNotFound(post_path.clone()))?;
        let metadata = MarkdownMetadata::new(&markdown_file)?;
        let content_raw = MarkdownMetadata::extract(&markdown_file)?;
        let mut content = String::new();
        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES);
        let parser = pulldown_cmark::Parser::new_ext(&content_raw, options);

        // let parser =
        //     pulldown_cmark::Parser::new_ext(&content_raw, options).map(|event| match event {
        //         Event::Start(Tag::Heading { .. }) => Event::InlineHtml("<h1 id=\"level1\">".into()),
        //         _ => event,
        //     });
        pulldown_cmark::html::push_html(&mut content, parser);

        Ok(Self { metadata, content })
    }

    #[instrument]
    // TODO: TEST
    pub async fn add_views_to_markdown(
        sqlite_conn: Connection,
        title: &str,
    ) -> Result<(), ServerError> {
        Self::increment_views(&sqlite_conn, title)
    }

    // TODO: TEST
    #[instrument(parent = None, skip(sqlite_conn))]
    pub async fn list_private_markdown_info(
        sqlite_conn: Connection,
        blog_config: Blog,
    ) -> Result<Vec<(MarkdownMetadata, PostInfo)>, ServerError> {
        info!("Starting reading path: {}", blog_config.path.clone());
        let paths = fs::read_dir(blog_config.path)
            .map_err(|e| {
                tracing::error!("folder of user: {} not found, error: {e}", blog_config.user)
            })
            .unwrap();

        let mut markdown_info: Vec<Option<(MarkdownMetadata, PostInfo)>> = Vec::new();

        for path in paths {
            let file_name = path
                .as_ref()
                .unwrap()
                .file_name()
                .clone()
                .into_string()
                .unwrap();

            info!("Starting reading markdown: {}", file_name);

            if path.as_ref().unwrap().file_type()?.is_dir() {
                warn!("{file_name} is a folder");
                continue;
            }

            let filepath = path.unwrap().path().display().to_string();
            let markdown_file = fs::read_to_string(&filepath)
                .map_err(|_| ServerError::PageNotFound(filepath.to_string()))?;
            let metadata_option = MarkdownMetadata::new(&markdown_file)
                .map_err(|e| tracing::warn!("{}", e.to_string()))
                .ok();

            match metadata_option {
                Some(metadata) => {
                    let post: PostInfo =
                        Self::find_or_create_post(&sqlite_conn, &metadata.filename)?;
                    markdown_info.push(Some((metadata, post)));
                }
                None => {
                    markdown_info.push(None);
                    continue;
                }
            }
        }

        Ok(markdown_info.into_iter().flatten().collect())
    }

    #[instrument]
    // TODO: TEST
    pub async fn list_markdown_info_of_a_post(
        sqlite_conn: Connection,
        full_file_path: String,
    ) -> Result<(MarkdownMetadata, PostInfo)> {
        let markdown_file = fs::read_to_string(full_file_path.clone())
            .map_err(|_| ServerError::PageNotFound(full_file_path.to_string()))?;
        let metadata = MarkdownMetadata::new(&markdown_file)?;
        let post: PostInfo = Self::find_or_create_post(&sqlite_conn, &metadata.filename)?;
        Ok((metadata, post))
    }
}

impl SqliteOperations for Markdown {}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct MarkdownMetadata {
    pub filename: String,
    pub title: String,
    pub subtitle: String,
    pub description: String,
    pub tags: Vec<String>,
    pub similar_posts: Option<Vec<String>>,
    pub date: String,
    pub finished: bool,
    pub image_preview: Option<String>,
    pub owner: Option<Uuid>,
}

impl MarkdownMetadata {
    #[instrument]
    // TODO: TEST
    fn new(input: &str) -> Result<Self, ServerError> {
        let result = YamlFrontMatter::parse::<MarkdownMetadata>(input)
            .map_err(|e| ServerError::YamlConvertionError(e.to_string()))?;
        Ok(result.metadata)
    }

    #[instrument]
    // TODO: TEST
    fn extract(string_output: &str) -> Result<String, ServerError> {
        let regex = regex::Regex::new(r"---((.|\n)*?)---")
            .map_err(|err| ServerError::InternalServer(err.to_string()))?;

        Ok(regex.replace(string_output, "").to_string())
    }
}

#[cfg(test)]
mod tests {}
