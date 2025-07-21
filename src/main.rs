use clap::Parser;
use cli::{Cli, Commands};
use prac_rs::sum;

mod cli;
mod task;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Add { files }) => {
            println!("Adding files: {:?}", files);
        }
        Some(Commands::Commit { message, all }) => {
            println!("Committing with message: {}", message);
            if *all {
                println!("Including all tracked files");
            }
        }
        Some(Commands::Status) => {
            println!("Showing status...");
        }
        None => {
            println!("No subcommand provided");
        }
    }

    let result = sum(2, 2);
    println!("The sum is: {}", result);
    task::my_task();
}
