mod cli;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let args = Cli::parse();

    dbg!(&args);
    dbg!(&args.command);

    match &args.command {
        Some(Commands::Add {
            files: _,
            recursive: _,
        }) => {
            println!("Adding a new item");
        }
        _ => {
            println!("Invalid command");
        }
    }
}
