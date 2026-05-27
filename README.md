# 🦀 Forge - High-Performance Rust Static Site Generator

A hyper-fast, concurrent static site generator (SSG) custom-built in Rust, optimized for developer notes, and fully tailored for deployment on platforms like Cloudflare Pages.

## ✨ Features

- ⚡ **Hyper-Fast Engine**: Compiled native Rust workspace pipelines (replaces heavy alternatives like Zola/Hugo).
- 🏗️ **Advanced Optimization**: Built-in AST-level HTML/CSS minification via `minify_html` and `lightningcss`.
- 📦 **Gzip Compression**: Automatic asset optimization pre-configured for Cloudflare edge deployment.
- 🔄 **Incremental Compilation**: Fingerprint-tracked dependency building via custom automation pipeline.
- 📝 **Developer-Friendly Writing**: Advanced Markdown parsing with syntax highlighting and front-matter tag systems.
- 📡 **Syndication Feeds**: Automated RSS feed generation (`feed.xml`) and SEO sitemaps.
- 🛠️ **Workspace Architecture**: Clean decoupling between core rendering logic (`forge`) and command-line execution (`forge-cli`).

## 🚀 Quick Start

### Local Development Pipeline
Since this project uses a custom automation pipeline, use the provided PowerShell script to build, compile, and generate test layouts:

```powershell
# Clone the repository
git clone [https://github.com/zqbx0/forge.git](https://github.com/zqbx0/forge.git)
cd forge

# Run complete pipeline (cleans cache, builds binaries, generates test posts, and outputs assets)
.\test.ps1 -All

```

### Build Website Distribution

To compile your production-ready static assets into the `public/` folder:

```powershell
.\test.ps1 -Build

```

### Create New Post

You can easily scaffold a new markdown post bundle via the custom CLI layer:

```powershell
.\target\debug\forge-cli.exe new "Your Post Title" --tags "rust,tutorial"

```

## 🛠️ Technical Architecture & Ecosystem

* **Core Renderer (`crates/forge`)**: Custom Rust markdown & layout mapping matrix.
* **CLI Driver (`crates/forge-cli`)**: Command-line arguments processing powered by `clap`.
* **Text & Parsing**: `pulldown_cmark` for high-speed Markdown; `pest` for grammars.
* **Asset Processing**: `lightningcss` for CSS, `minify_html` & `minify_js` for speed-of-light size reduction.
* **Date & Time**: Comprehensive timezone tracking using `chrono` and `chrono_tz`.
* **Concurrency**: Parallel iteration and thread pooling driven by `rayon`.

## 📂 Project Workspace Layout

```text
D:\Projects\forge\
├── Cargo.toml                  # Workspace master configuration
├── test.ps1                    # Automation test suite pipeline
├── crates/
│   ├── forge/                  # Core rendering engine library (lib.rs, markdown.rs)
│   └── forge-cli/              # CLI entrance & command parameters (main.rs)
├── templates/                  # Skin layout templates (index.html, archive.html)
│   └── assets/                 # Pure CSS, JS and raw UI styling sheets
└── static/                     # Pass-through global static files & media

```

## 🌐 Distribution & Deployment

* **Hosting Platform**: Cloudflare Pages / GitHub Actions CI
* **Main Site**: https://rust-blog.pages.dev
* **GitHub Repository**: https://github.com/zqbx0/forge
* **RSS Feed Matrix**: https://rust-blog.pages.dev/feed.xml
* **Sitemap Matrix**: https://rust-blog.pages.dev/sitemap.xml
* **Production Asset Directory**: `public/`

## 📬 Contact

* GitHub: [@zqbx0](https://github.com/zqbx0)
* Blog: https://rust-blog.pages.dev

## 📄 License

MIT License

```

