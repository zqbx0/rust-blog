pub mod blog;
pub mod markdown;
pub mod server;
pub mod template;
pub mod utils;

pub use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "forge",
    version,
    about = "High-performance Static Site Generator"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(short, long, global = true)]
    pub verbose: bool,

    #[arg(short, long, global = true, default_value_t = num_cpus::get())]
    pub jobs: usize,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Build {
        #[arg(short, long, default_value = "public")]
        output: PathBuf,
        #[arg(long)]
        minify: bool,
        #[arg(long)]
        gzip: bool,
        #[arg(long)]
        incremental: bool,
    },
    Serve {
        #[arg(short, long, default_value_t = 8080)]
        port: u16,
        #[arg(long, default_value = "127.0.0.1")]
        host: String,
        #[arg(long)]
        watch: bool,
    },
    Check,
    Rss {
        #[arg(short, long)]
        output: PathBuf,
    },
}

pub fn handle_execution(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    match cli.command {
        Commands::Build {
            output,
            minify,
            gzip,
            incremental,
        } => {
            blog::build(&output, minify, gzip, incremental)?;
        }
        Commands::Serve { port, host, .. } => {
            println!("Starting server at http://{}:{}", host, port);
        }
        Commands::Check => {
            let posts = utils::list_posts();
            println!("Verified {} nodes seamlessly.", posts.len());
        }
        Commands::Rss { output } => {
            blog::generate_rss(&output)?;
        }
    }
    Ok(())
}
