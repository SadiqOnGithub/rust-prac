use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(name = "myapp")] // <- "When showing help, call this program 'myapp'"
#[command(about = "A simple example program")] // <- "Show this description in --help"
pub struct Cli {
    #[arg(short, long)]
    pub name: String,
}
