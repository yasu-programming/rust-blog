use clap::Parser;
use rust_blog::{Cli, FileStorage, execute_command};
use std::process;

fn main() {
    let cli = Cli::parse();
    let storage = FileStorage::new("data".to_string());

    if let Err(e) = execute_command(cli, &storage) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
