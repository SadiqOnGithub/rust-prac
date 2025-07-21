use clap::Parser;

// When you write this:
#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    name: String,

    #[arg(short, long)]
    age: u32,
}

// Clap's #[derive(Parser)] generates code similar to this:
impl Cli {
    fn parse() -> Self {
        let args: Vec<String> = std::env::args().collect();

        // All that painful manual parsing we wrote above
        // gets generated automatically here!

        let mut name = String::new();
        let mut age = 0u32;

        // ... (imagine all the parsing logic here)

        Cli { name, age }
    }

    fn parse_from<I, T>(args: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<std::ffi::OsString> + Clone,
    {
        // Even more complex parsing logic
        unimplemented!("This is just for illustration")
    }
}

fn main() {
    // This one line replaces all our manual parsing!
    let cli = Cli::parse();
    println!("Hello {}, you are {} years old!", cli.name, cli.age);
}
