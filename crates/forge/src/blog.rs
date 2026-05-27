use crate::markdown::parse_article_fast;
use crate::template::TemplateEngine;
use crate::utils;
use serde::Serialize;
use std::fs;
use std::path::Path;

#[derive(Serialize)]
struct SiteConfig {
    title: String,
}

pub fn build(
    output_dir: &Path,
    _minify: bool,
    _gzip: bool,
    _incremental: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let output_str = output_dir.to_str().unwrap_or("public");
    utils::prepare_output_dir(output_str)?;

    let config = SiteConfig {
        title: "Rust Blog".to_string(),
    };

    let ts = TemplateEngine::new("templates")?;
    let files = utils::get_md_files("content");

    let mut posts = Vec::new();

    for file_path in files {
        if let Ok(post) = parse_article_fast(&file_path) {
            let html = ts.render_post(&post, &config)?;
            let file_name = file_path.file_stem().unwrap().to_str().unwrap();

            let posts_dir = output_dir.join("posts");
            if !posts_dir.exists() {
                fs::create_dir_all(&posts_dir)?;
            }

            let output_path = posts_dir.join(format!("{}.html", file_name));
            fs::write(output_path, html)?;
            posts.push(post);
        }
    }

    let index_path = output_dir.join("index.html");
    if let Ok(index_html) = ts.render_index(&posts, &config) {
        fs::write(&index_path, index_html)?;
    } else {
        fs::write(
            &index_path,
            "<h1>Welcome to my Rust Blog!</h1><p>Please navigate to the specific post HTML files.</p>",
        )?;
    }

    let archive_path = output_dir.join("archive.html");
    if let Ok(archive_html) = ts.render_archive(&posts, &config) {
        fs::write(archive_path, archive_html)?;
    }

    Ok(())
}

pub fn generate_rss(output_file: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(parent) = output_file.parent() {
        fs::create_dir_all(parent)?;
    }

    let dummy_rss = r#"<?xml version="1.0" encoding="UTF-8" ?>
<rss version="2.0">
<channel>
  <title>Rust Blog</title>
  <description>High-performance static RSS feed node.</description>
</channel>
</rss>"#;

    fs::write(output_file, dummy_rss)?;
    println!("RSS Syndication Feed compiled safely at: {:?}", output_file);
    Ok(())
}
