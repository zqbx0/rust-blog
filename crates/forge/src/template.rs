use crate::markdown::Post;
use serde::Serialize;
use std::path::Path;
use tera::{Context, Tera};

pub struct TemplateEngine {
    tera: Tera,
}

impl TemplateEngine {
    pub fn new(template_dir: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let template_path = if Path::new(template_dir).exists() {
            format!("{}/**/*.html", template_dir)
        } else {
            String::new()
        };

        let tera = if !template_path.is_empty() {
            Tera::new(&template_path)?
        } else {
            Tera::default()
        };

        Ok(Self { tera })
    }

    pub fn render_index<C: Serialize>(
        &self,
        posts: &[Post],
        config: &C,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut context = Context::new();
        let published_posts: Vec<&Post> = posts.iter().filter(|p| !p.meta.draft).collect();

        context.insert("posts", &published_posts);
        context.insert("config", config);
        context.insert(
            "generated_at",
            &chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        );
        context.insert("version", env!("CARGO_PKG_VERSION"));

        self.tera
            .render("index.html", &context)
            .map_err(|e| e.into())
    }

    pub fn render_post<C: Serialize>(
        &self,
        post: &Post,
        config: &C,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut context = Context::new();
        context.insert("post", post);
        context.insert("config", config);
        context.insert(
            "generated_at",
            &chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        );
        context.insert("version", env!("CARGO_PKG_VERSION"));

        self.tera
            .render("post.html", &context)
            .map_err(|e| e.into())
    }

    pub fn render_archive<C: Serialize>(
        &self,
        posts: &[Post],
        config: &C,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut context = Context::new();
        let published_posts: Vec<&Post> = posts.iter().filter(|p| !p.meta.draft).collect();

        context.insert("posts", &published_posts);
        context.insert("config", config);
        context.insert(
            "generated_at",
            &chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        );
        context.insert("version", env!("CARGO_PKG_VERSION"));

        self.tera
            .render("archive.html", &context)
            .map_err(|e| e.into())
    }
}
