use clap::Parser;
use forge::{Cli, handle_execution};

fn main() {
    let cli = Cli::parse();

    if let Err(e) = handle_execution(cli) {
        eprintln!("Execution error: {}", e);
        std::process::exit(1);
    }
}
