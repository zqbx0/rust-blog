use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "blog")]
#[command(author, version, about = "A fast, parallel static site generator written in Rust", long_about = None)]
pub struct Cli {
    #[arg(short, long, default_value_t = num_cpus::get())]
    pub jobs: usize,

    #[subcommand]
    pub command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    Build,
    Serve {
        #[arg(short, long, default_value = "127.0.0.1:8080")]
        addr: String,
    },
    Info,
    Clean,
    Check,
    Rss {
        #[arg(short, long, default_value = "public/feed.xml")]
        output: String,
    },
}
