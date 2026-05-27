use crate::markdown::parse_article_fast;
use chrono::NaiveDate;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct PostInfo {
    pub slug: String,
    pub title: String,
    pub date: Option<NaiveDate>,
    pub draft: bool,
}

pub fn list_posts() -> Vec<PostInfo> {
    let mut posts = Vec::new();
    let posts_dir = Path::new("content");
    if !posts_dir.exists() {
        return posts;
    }

    if let Ok(entries) = fs::read_dir(posts_dir) {
        for entry in entries.flatten() {
            let path = entry.path();

            if path.extension().is_none_or(|ext| ext != "md") {
                continue;
            }

            if let Ok(post) = parse_article_fast(&path) {
                let parsed_date = NaiveDate::parse_from_str(&post.meta.date, "%Y-%m-%d").ok();
                posts.push(PostInfo {
                    slug: post.meta.slug,
                    title: post.meta.title,
                    date: parsed_date,
                    draft: post.meta.draft,
                });
            }
        }
    }

    posts.sort_by_key(|b| std::cmp::Reverse(b.date));
    posts
}

pub fn count_posts() -> usize {
    list_posts().iter().filter(|p| !p.draft).count()
}

pub fn count_drafts() -> usize {
    list_posts().iter().filter(|p| p.draft).count()
}

pub fn prepare_output_dir(path: &str) -> std::io::Result<()> {
    if Path::new(path).exists() {
        fs::remove_dir_all(path)?;
    }
    fs::create_dir_all(path)
}

pub fn get_md_files(path: &str) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_file() && p.extension().is_some_and(|ext| ext == "md") {
                files.push(p);
            }
        }
    }
    files
}
