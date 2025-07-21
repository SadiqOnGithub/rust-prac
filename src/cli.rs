use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "git")]
#[command(about = "A fictional versioning CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Adds files to the repository
    Add {
        /// Files to add
        #[arg(required = true)]
        files: Vec<String>,
    },
    /// Commits changes
    Commit {
        /// Commit message
        #[arg(short, long)]
        message: String,

        /// All tracked files
        #[arg(short, long)]
        all: bool,
    },
    /// Shows the status
    Status,
}
