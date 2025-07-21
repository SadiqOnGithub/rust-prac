// Attributes are metadata that tell the compiler what to do

// 1. Built-in derive attributes
#[derive(Debug)] // Compiler generates debug printing code
struct Point {
    x: i32,
    y: i32,
}

// This is equivalent to manually writing:
// impl std::fmt::Debug for Point {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "Point {{ x: {}, y: {} }}", self.x, self.y)
//     }
// }

// 2. Multiple derives
#[derive(Debug, Clone, PartialEq)] // Generate 3 different implementations
struct Person {
    name: String,
}

// 3. Custom attributes (like clap's)
use clap::Parser;

#[derive(Parser)] // This tells clap's macro to generate parsing code
struct Config {
    #[arg(short)] // This tells clap: "make this available as -n"
    name: String,
}

// What clap actually does:
// 1. Reads your struct definition
// 2. Looks at the #[arg(...)] attributes
// 3. Generates a ton of parsing code based on those attributes
// 4. Creates methods like parse(), try_parse(), etc.

fn main() {
    let p = Point { x: 1, y: 2 };
    println!("{:?}", p); // This works because of #[derive(Debug)]

    let config = Config::parse(); // This works because of #[derive(Parser)]
    println!("Name: {}", config.name);
}
