use crate::models::Article;
use crate::storage::FileStorage;
use clap::{Parser, Subcommand};
use std::io::{self, Write};

#[derive(Parser)]
#[command(name = "rust-blog")]
#[command(about = "A simple blog CLI written in Rust")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Create {
        #[arg(short, long)]
        title: String,
        #[arg(short, long)]
        content: String,
    },
    List,
    Show {
        #[arg(value_name = "ID")]
        id: String,
    },
    Edit {
        #[arg(value_name = "ID")]
        id: String,
        #[arg(short, long)]
        title: Option<String>,
        #[arg(short, long)]
        content: Option<String>,
    },
    Delete {
        #[arg(value_name = "ID")]
        id: String,
    },
}

pub fn execute_command(cli: Cli, storage: &FileStorage) -> Result<(), Box<dyn std::error::Error>> {
    match cli.command {
        Commands::Create { title, content } => {
            let article = Article::new(title, content);
            storage.save_article(&article)?;
            println!("Created article with ID: {}", article.id);
        }
        Commands::List => {
            let articles = storage.list_articles()?;
            if articles.is_empty() {
                println!("No articles found.");
            } else {
                println!("Articles:");
                for (id, title) in articles {
                    println!("  {} - {}", id, title);
                }
            }
        }
        Commands::Show { id } => match storage.load_article(&id) {
            Ok(article) => {
                println!("ID: {}", article.id);
                println!("Title: {}", article.title);
                println!(
                    "Created: {}",
                    article.created_at.format("%Y-%m-%d %H:%M:%S")
                );
                println!(
                    "Updated: {}",
                    article.updated_at.format("%Y-%m-%d %H:%M:%S")
                );
                println!("Content:");
                println!("{}", article.content);
            }
            Err(e) => {
                eprintln!("Error loading article: {}", e);
                return Err(e.into());
            }
        },
        Commands::Edit { id, title, content } => {
            if title.is_none() && content.is_none() {
                eprintln!("Please provide at least --title or --content to edit");
                return Ok(());
            }

            match storage.load_article(&id) {
                Ok(mut article) => {
                    article.update(title, content);
                    storage.save_article(&article)?;
                    println!("Updated article: {}", article.id);
                }
                Err(e) => {
                    eprintln!("Error loading article: {}", e);
                    return Err(e.into());
                }
            }
        }
        Commands::Delete { id } => {
            print!("Are you sure you want to delete article '{}'? (y/N): ", id);
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            if input.trim().to_lowercase() == "y" || input.trim().to_lowercase() == "yes" {
                storage.delete_article(&id)?;
                println!("Deleted article: {}", id);
            } else {
                println!("Deletion cancelled.");
            }
        }
    }
    Ok(())
}
