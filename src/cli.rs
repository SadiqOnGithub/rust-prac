use clap::{Parser, Subcommand};
use std::{fmt::Debug, path::PathBuf};

#[derive(Parser, Debug)]
#[command(name = "git")]
#[command(about = "A fictional versioning CLI")]
pub struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Adds files to the repository
    Add {
        #[arg(short, long, default_value_t = true)]
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
