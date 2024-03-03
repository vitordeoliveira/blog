use serde::Deserialize;
use yaml_front_matter::YamlFrontMatter;

use crate::error::{Error, Result};

#[derive(Deserialize)]
pub struct MarkdownMetadata {
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    pub similar_posts: Vec<String>,
    pub date: String,
}

impl MarkdownMetadata {
    pub fn new(input: &str) -> Result<Self> {
        let result = YamlFrontMatter::parse::<MarkdownMetadata>(input)
            .map_err(|_| Error::InternalServerError("Error on YamlFrontMatter".to_string()))?;

        Ok(result.metadata)
    }

    pub fn extract(string_output: &str) -> Result<String> {
        let regex = regex::Regex::new(r"---((.|\n)*?)---")
            .map_err(|err| Error::InternalServerError(err.to_string()))?;

        Ok(regex.replace(string_output, "").to_string())
    }
}
