use std::env;

fn main() {
    // This is ALL Rust gives you - a vector of strings
    let args: Vec<String> = env::args().collect();

    println!("Raw arguments: {:?}", args);

    // If you run: cargo run -- --name Alice --age 25
    // You get: ["target/debug/myprogram", "--name", "Alice", "--age", "25"]

    // Now YOU have to figure out what each string means!
}
