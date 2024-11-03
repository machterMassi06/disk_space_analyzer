mod file_tree;
mod print_tree;
mod size;

use clap::{Parser, Subcommand};
use file_tree::FileTree;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show the disk usage tree for the given path
    Usage {
        /// (default '.')
        path: Option<PathBuf>,
    },
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Usage { path } => {
            let path = path.as_deref().unwrap_or(Path::new("."));
            FileTree::new(path)?.show();
        }
    }
    Ok(())
}
