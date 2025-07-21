use clap::Parser;

// These attributes are just configuration data for clap:

#[derive(Parser)]
#[command(name = "git")] // Sets program_name = "git"
#[command(about = "Version control")] // Sets description = "Version control"
#[command(version = "1.0")] // Sets version = "1.0"
struct Cli {
    #[arg(short, long)]
    file: String,
}

// This is roughly equivalent to configuring clap manually like this:
//
// let app = Command::new("git")
//     .about("Version control")
//     .version("1.0")
//     .arg(Arg::new("file")
//         .short('f')
//         .long("file")
//         .required(true));

// The attributes are just a convenient way to write configuration
// instead of using the builder pattern

fn main() {
    let cli = Cli::parse();

    // When someone runs: your_program --help
    // They'll see:
    //
    // git 1.0
    // Version control
    //
    // Usage: git [OPTIONS] --file <FILE>
    //
    // Options:
    //   -f, --file <FILE>
    //   -h, --help         Print help information
    //   -V, --version      Print version information

    println!("Processing file: {}", cli.file);
}
