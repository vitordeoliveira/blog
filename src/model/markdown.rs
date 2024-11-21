use std::fs;

use anyhow::Result;
use pulldown_cmark::Options;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use tracing::instrument;
use uuid::Uuid;
use yaml_front_matter::YamlFrontMatter;

use crate::error::ServerError;

use super::sqlite::{PostInfo, SqliteOperations};

pub struct Markdown {
    pub metadata: MarkdownMetadata,
    pub content: String,
}

impl Markdown {
    #[instrument]
    // TODO: TEST
    pub fn new(postname: String) -> Result<Self> {
        let file = format!("./blogpost/{}.md", &postname);
        let markdown_file =
            fs::read_to_string(file).map_err(|_| ServerError::PageNotFound(postname.clone()))?;
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

    #[instrument]
    // TODO: TEST
    pub async fn list_private_markdown_info(
        sqlite_conn: Connection,
        user_id: Uuid,
    ) -> Result<Vec<(MarkdownMetadata, PostInfo)>> {
        let paths = fs::read_dir("./blogpost").unwrap();

        let mut markdown_info: Vec<(MarkdownMetadata, PostInfo)> = Vec::new();

        for path in paths {
            let filepath = path.unwrap().path().display().to_string();
            let markdown_file = fs::read_to_string(&filepath)
                .map_err(|_| ServerError::PageNotFound(filepath.to_string()))?;
            let metadata = MarkdownMetadata::new(&markdown_file)?;
            if let Some(owner) = metadata.owner {
                if user_id == owner {
                    let post: PostInfo =
                        Self::find_or_create_post(&sqlite_conn, &metadata.filename)?;
                    markdown_info.push((metadata, post));
                }
            }
        }

        Ok(markdown_info)
    }

    #[instrument]
    // TODO: TEST
    pub async fn list_markdown_info(
        sqlite_conn: Connection,
    ) -> Result<Vec<(MarkdownMetadata, PostInfo)>> {
        let paths = fs::read_dir("./blogpost").unwrap();

        let mut markdown_info: Vec<(MarkdownMetadata, PostInfo)> = Vec::new();

        for path in paths {
            let filepath = path.unwrap().path().display().to_string();
            let markdown_file = fs::read_to_string(&filepath)
                .map_err(|_| ServerError::PageNotFound(filepath.to_string()))?;
            let metadata = MarkdownMetadata::new(&markdown_file)?;

            if metadata.owner.is_some() {
                continue;
            }

            let post: PostInfo = Self::find_or_create_post(&sqlite_conn, &metadata.filename)?;

            markdown_info.push((metadata, post));
        }

        Ok(markdown_info)
    }

    #[instrument]
    // TODO: TEST
    pub async fn list_markdown_info_of_post(
        sqlite_conn: Connection,
        filepath: String,
    ) -> Result<(MarkdownMetadata, PostInfo)> {
        let markdown_file = fs::read_to_string(format!("./blogpost/{}", &filepath))
            .map_err(|_| ServerError::PageNotFound(filepath.to_string()))?;
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
    pub similar_posts: Vec<String>,
    pub date: String,
    pub finished: bool,
    pub image_preview: Option<String>,
    pub owner: Option<Uuid>,
}

impl MarkdownMetadata {
    #[instrument]
    // TODO: TEST
    fn new(input: &str) -> Result<Self> {
        let result = YamlFrontMatter::parse::<MarkdownMetadata>(input)
            .map_err(|e| ServerError::InternalServer(format!("Error on YamlFrontMatter: {e}")))?;
        Ok(result.metadata)
    }

    #[instrument]
    // TODO: TEST
    fn extract(string_output: &str) -> Result<String> {
        let regex = regex::Regex::new(r"---((.|\n)*?)---")
            .map_err(|err| ServerError::InternalServer(err.to_string()))?;

        Ok(regex.replace(string_output, "").to_string())
    }
}
