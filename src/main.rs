use clap::Parser;
use prac_rs::sum;

mod cli;
mod task;

fn main() {
    let cli = dbg!(cli::Cli::parse());

    println!("Hello, {}!", cli.name);

    let result = sum(2, 2);
    println!("The sum is: {}", result);
    task::my_task();
}
