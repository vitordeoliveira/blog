use std::fs;

use serde::Deserialize;
use yaml_front_matter::YamlFrontMatter;

use crate::error::{Error, Result};

pub struct Markdown {
    pub metadata: MarkdownMetadata,
    pub content: String,
}

// let markdown_input =
//     fs::read_to_string(file).map_err(|_| Error::PageNotFound(postname.clone()))?;
// let metadata = MarkdownMetadata::new(&markdown_input)?;
// let mut html_output = String::new();
// let res = MarkdownMetadata::extract(&markdown_input)?;

impl Markdown {
    pub fn new(postname: String) -> Result<Self> {
        let file = format!("./blogpost/{}.md", &postname);
        let markdown_file =
            fs::read_to_string(file).map_err(|_| Error::PageNotFound(postname.clone()))?;
        let metadata = MarkdownMetadata::new(&markdown_file)?;
        let content_raw = MarkdownMetadata::extract(&markdown_file)?;
        let mut content = String::new();
        let parser = pulldown_cmark::Parser::new(&content_raw);
        pulldown_cmark::html::push_html(&mut content, parser);

        Ok(Self { metadata, content })
    }
}

#[derive(Deserialize)]
pub struct MarkdownMetadata {
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    pub similar_posts: Vec<String>,
    pub date: String,
}

impl MarkdownMetadata {
    fn new(input: &str) -> Result<Self> {
        let result = YamlFrontMatter::parse::<MarkdownMetadata>(input)
            .map_err(|_| Error::InternalServerError("Error on YamlFrontMatter".to_string()))?;

        Ok(result.metadata)
    }

    fn extract(string_output: &str) -> Result<String> {
        let regex = regex::Regex::new(r"---((.|\n)*?)---")
            .map_err(|err| Error::InternalServerError(err.to_string()))?;

        Ok(regex.replace(string_output, "").to_string())
    }
}
