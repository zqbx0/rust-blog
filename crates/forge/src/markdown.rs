use memchr::memmem;
use serde::{Deserialize, Serialize, ser::SerializeStruct};
use std::fs;
use std::path::Path;
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ArticleMeta {
    pub title: String,
    pub date: String,
    pub updated: Option<String>,
    pub draft: bool,
    pub tags: Vec<String>,
    pub slug: String,
    pub series: Option<String>,
    pub summary: Option<String>,
    pub cover: Option<String>,
    pub authors: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Post {
    pub meta: ArticleMeta,
    pub html_content: Arc<String>,
    pub reading_time: usize,
    pub word_count: usize,
}

impl Serialize for Post {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Post", 4)?;
        state.serialize_field("meta", &self.meta)?;
        state.serialize_field("html_content", self.html_content.as_str())?;
        state.serialize_field("reading_time", &self.reading_time)?;
        state.serialize_field("word_count", &self.word_count)?;
        state.end()
    }
}

fn fast_markdown_to_html(markdown: &str) -> Arc<String> {
    let mut options = pulldown_cmark::Options::empty();
    options.insert(pulldown_cmark::Options::ENABLE_STRIKETHROUGH);
    options.insert(pulldown_cmark::Options::ENABLE_TABLES);
    options.insert(pulldown_cmark::Options::ENABLE_TASKLISTS);
    options.insert(pulldown_cmark::Options::ENABLE_FOOTNOTES);

    let parser = pulldown_cmark::Parser::new_ext(markdown, options);
    let mut html = String::with_capacity(markdown.len() * 2);
    pulldown_cmark::html::push_html(&mut html, parser);
    Arc::new(html)
}

pub fn parse_article_fast(path: &Path) -> Result<Post, String> {
    let content = fs::read_to_string(path).map_err(|e| format!("File read failed: {}", e))?;

    let frontmatter_end = match memmem::find(content.as_bytes(), b"+++") {
        Some(pos) => match memmem::find(&content.as_bytes()[pos + 3..], b"+++") {
            Some(second_pos) => pos + 3 + second_pos,
            None => {
                return Err("Invalid Front Matter format: missing closing delimiter".to_string());
            }
        },
        None => return Err("Invalid Front Matter format: missing initial delimiter".to_string()),
    };

    let frontmatter = &content[3..frontmatter_end];
    let markdown = &content[frontmatter_end + 3..];

    let meta: ArticleMeta =
        toml::from_str(frontmatter.trim()).map_err(|e| format!("TOML parsing failed: {}", e))?;
    let html_content = fast_markdown_to_html(markdown.trim());
    let word_count = markdown.split_whitespace().count();
    let reading_time = (word_count as f32 / 200.0).ceil() as usize;

    Ok(Post {
        meta,
        html_content,
        reading_time,
        word_count,
    })
}
