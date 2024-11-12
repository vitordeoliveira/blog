use std::fs;

use anyhow::Result;
use pulldown_cmark::Options;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use yaml_front_matter::YamlFrontMatter;

use crate::error::ServerError;

pub struct Markdown {
    pub metadata: MarkdownMetadata,
    pub content: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PostInfo {
    id: String,
    pub views: u16,
}

#[derive(Debug)]
struct Post {
    // TODO: maybe remove this id field, for now is really necessary
    id: u16,
    title: String,
    views: u16,
}

trait SqliteOperations {
    fn find_or_create_post(sqlite_conn: &Connection, title: &str) -> Result<PostInfo, ServerError>;
    fn increment_views(sqlite_conn: &Connection, title: &str) -> Result<(), ServerError>;
}

impl SqliteOperations for Markdown {
    fn find_or_create_post(sqlite_conn: &Connection, title: &str) -> Result<PostInfo, ServerError> {
        let mut stmt =
            sqlite_conn.prepare_cached("SELECT id, title, views FROM posts WHERE title = ?1")?;

        let post = stmt.query_row([title], |row| {
            Ok(Post {
                id: row.get(0)?,
                title: row.get(1)?,
                views: row.get(2)?,
            })
        });

        Ok(match post {
            Ok(post) => PostInfo {
                id: post.title,
                views: post.views,
            },
            Err(rusqlite::Error::QueryReturnedNoRows) => {
                sqlite_conn.execute("INSERT INTO posts (title) values (?1)", params![title])?;

                let post = stmt.query_row([title], |row| {
                    Ok(Post {
                        id: row.get(0)?,
                        title: row.get(1)?,
                        views: row.get(2)?,
                    })
                })?;

                PostInfo {
                    id: post.title,
                    views: post.views,
                }
            }
            Err(e) => return Err(ServerError::DBError(e)),
        })
    }

    fn increment_views(sqlite_conn: &Connection, title: &str) -> Result<(), ServerError> {
        sqlite_conn.execute(
            "UPDATE posts SET views = views + 1 WHERE title = ?1",
            params![title],
        )?;

        Ok(())
    }
}

impl Markdown {
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

    pub async fn add_views_to_markdown(title: &str) -> Result<(), ServerError> {
        let sqlite_conn = Connection::open("./data/blog.sqlite")?;
        Self::increment_views(&sqlite_conn, title)
    }

    pub async fn list_markdown_info() -> Result<Vec<(MarkdownMetadata, PostInfo)>> {
        let paths = fs::read_dir("./blogpost").unwrap();

        let mut markdown_info: Vec<(MarkdownMetadata, PostInfo)> = Vec::new();

        for path in paths {
            let filepath = path.unwrap().path().display().to_string();
            let markdown_file = fs::read_to_string(&filepath)
                .map_err(|_| ServerError::PageNotFound(filepath.to_string()))?;
            let metadata = MarkdownMetadata::new(&markdown_file)?;
            // let post: PostInfo = Self::get_views_or_create(&metadata.filename).await?;

            let sqlite_conn = Connection::open("./data/blog.sqlite")?;
            let post: PostInfo = Self::find_or_create_post(&sqlite_conn, &metadata.filename)?;

            markdown_info.push((metadata, post));
        }

        Ok(markdown_info)
    }

    pub async fn list_markdown_info_of_post(
        filepath: String,
    ) -> Result<(MarkdownMetadata, PostInfo)> {
        let markdown_file = fs::read_to_string(format!("./blogpost/{}", &filepath))
            .map_err(|_| ServerError::PageNotFound(filepath.to_string()))?;
        let metadata = MarkdownMetadata::new(&markdown_file)?;
        // TODO: ok, I am creating connections all the time, use the app state insead
        let sqlite_conn = Connection::open("./data/blog.sqlite")?;
        let post: PostInfo = Self::find_or_create_post(&sqlite_conn, &metadata.filename)?;
        Ok((metadata, post))
    }
}

#[derive(Debug, Deserialize)]
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
}

impl MarkdownMetadata {
    fn new(input: &str) -> Result<Self> {
        let result = YamlFrontMatter::parse::<MarkdownMetadata>(input)
            .map_err(|_| ServerError::InternalServer("Error on YamlFrontMatter".to_string()))?;

        Ok(result.metadata)
    }

    fn extract(string_output: &str) -> Result<String> {
        let regex = regex::Regex::new(r"---((.|\n)*?)---")
            .map_err(|err| ServerError::InternalServer(err.to_string()))?;

        Ok(regex.replace(string_output, "").to_string())
    }
}
