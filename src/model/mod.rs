use std::fs;

use pulldown_cmark::Options;
use serde::{Deserialize, Serialize};
use yaml_front_matter::YamlFrontMatter;

use crate::{
    config::database::firestore,
    error::{Error, Result},
};

pub struct Markdown {
    pub metadata: MarkdownMetadata,
    pub content: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PostInfo {
    id: String,
    pub views: u16,
}

const COLLECTION_NAME: &str = "posts";

impl Markdown {
    pub fn new(postname: String) -> Result<Self> {
        let file = format!("./blogpost/{}.md", &postname);
        let markdown_file =
            fs::read_to_string(file).map_err(|_| Error::PageNotFound(postname.clone()))?;
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

    async fn get_views_or_create(title: &str) -> Result<PostInfo> {
        let db = firestore().await;

        let post = db
            .fluent()
            .select()
            .by_id_in(COLLECTION_NAME)
            .obj()
            .one(title)
            .await
            .map_err(|_| Error::InternalServer("Error on query".to_string()))?;

        Ok(match post {
            Some(data) => data,
            None => {
                db.fluent()
                    .insert()
                    .into(COLLECTION_NAME)
                    .document_id(title)
                    .object(&PostInfo {
                        id: title.to_string(),
                        views: 0,
                    })
                    .execute::<PostInfo>()
                    .await
                    .unwrap();

                PostInfo {
                    id: title.to_string(),
                    views: 0,
                }
            }
        })
    }

    pub async fn add_views_to_markdown(title: &str) -> Result<()> {
        let db = firestore().await;
        let postview = Markdown::get_views_or_create(title).await?;

        db.fluent()
            .update()
            .in_col(COLLECTION_NAME)
            .document_id(title)
            .object(&PostInfo {
                views: postview.views + 1,
                ..postview
            })
            .execute::<PostInfo>()
            .await
            .unwrap();
        Ok(())
    }

    pub async fn list_markdown_info() -> Result<Vec<(MarkdownMetadata, PostInfo)>> {
        let paths = fs::read_dir("./blogpost").unwrap();

        let mut markdown_info: Vec<(MarkdownMetadata, PostInfo)> = Vec::new();

        for path in paths {
            let filepath = path.unwrap().path().display().to_string();
            let markdown_file = fs::read_to_string(&filepath)
                .map_err(|_| Error::PageNotFound(filepath.to_string()))?;
            let metadata = MarkdownMetadata::new(&markdown_file)?;
            let post: PostInfo = Self::get_views_or_create(&metadata.filename).await?;

            markdown_info.push((metadata, post));
        }

        Ok(markdown_info)
    }

    pub async fn list_markdown_info_of_post(
        filepath: String,
    ) -> Result<(MarkdownMetadata, PostInfo)> {
        let markdown_file = fs::read_to_string(format!("./blogpost/{}", &filepath))
            .map_err(|_| Error::PageNotFound(filepath.to_string()))?;
        let metadata = MarkdownMetadata::new(&markdown_file)?;
        let post: PostInfo = Self::get_views_or_create(&metadata.filename).await?;

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
            .map_err(|_| Error::InternalServer("Error on YamlFrontMatter".to_string()))?;

        Ok(result.metadata)
    }

    fn extract(string_output: &str) -> Result<String> {
        let regex = regex::Regex::new(r"---((.|\n)*?)---")
            .map_err(|err| Error::InternalServer(err.to_string()))?;

        Ok(regex.replace(string_output, "").to_string())
    }
}
