use clap::{Parser, Subcommand};
use std::fmt::Debug;

#[derive(Parser, Debug)]
#[command(name = "git")]
#[command(about = "A fictional versioning CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Adds files to the repository
    Add {
        #[clap(short, long, default_value_t = true)]
        recursive: bool,
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
