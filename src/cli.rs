use clap::Parser;

#[derive(Parser)]
// #[command(version, about, long_about = None)]
#[command(about = "A simple example program")] // <- "Show this description in --help"
pub struct Cli {
    // #[arg(short, long)]
    pub name: String,
}
