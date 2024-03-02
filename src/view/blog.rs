use std::fs;

use askama::Template;
use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse},
};
use axum_macros::debug_handler;
use serde::Deserialize;
use yaml_front_matter::YamlFrontMatter;

use crate::error::{self, Error, Result, ResultPath};

#[derive(Template)]
#[template(path = "blog.html")]
struct BlogTemplate {
    title: String,
    description: String,
    tags: Vec<String>,
    similar_posts: Vec<String>,
    date: String,
    markdown_html: String,
}

#[derive(Deserialize)]
struct Metadata {
    title: String,
    description: String,
    tags: Vec<String>,
    similar_posts: Vec<String>,
    date: String,
}

impl Metadata {
    fn new(input: &str) -> Result<Self> {
        let result = YamlFrontMatter::parse::<Metadata>(input)
            .map_err(|_| Error::InternalServerError("Error on YamlFrontMatter".to_string()))?;

        Ok(result.metadata)
    }

    fn extract(string_output: &str) -> Result<String> {
        let regex = regex::Regex::new(r"---((.|\n)*?)---")
            .map_err(|err| Error::InternalServerError(err.to_string()))?;

        Ok(regex.replace(string_output, "").to_string())
    }
}

pub async fn blog_view(ResultPath(postname): ResultPath<String>) -> Result<impl IntoResponse> {
    let file = format!("./blogpost/{}.md", &postname);
    let markdown_input =
        fs::read_to_string(file).map_err(|_| Error::PageNotFound(postname.clone()))?;
    let Metadata {
        title,
        description,
        tags,
        similar_posts,
        date,
    } = Metadata::new(&markdown_input)?;

    let mut html_output = String::new();
    let res = Metadata::extract(&markdown_input)?;
    let parser = pulldown_cmark::Parser::new(&res);

    pulldown_cmark::html::push_html(&mut html_output, parser);
    let root = BlogTemplate {
        title,
        description,
        tags,
        similar_posts,
        date,
        markdown_html: html_output,
    };

    let html = match root.render() {
        Ok(html) => html,
        Err(_) => {
            return Err(error::Error::InternalServerError(
                "Error on render".to_string(),
            ))
        }
    };

    Ok((StatusCode::OK, Html(html)))
}
