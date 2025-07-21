use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("Raw arguments: {:?}", args);

    // Manual parsing - this is painful!
    let mut name = String::new();
    let mut age = 0u32;

    let mut i = 1; // Skip program name at index 0
    while i < args.len() {
        dbg!(i);
        dbg!(args[i].as_str());
        match args[i].as_str() {
            "--name" | "-n" => {
                if i + 1 < args.len() {
                    name = args[i + 1].clone();
                    i += 2; // Skip the value
                } else {
                    eprintln!("Error: --name requires a value");
                    return;
                }
            }
            "--age" | "-a" => {
                if i + 1 < args.len() {
                    match args[i + 1].parse::<u32>() {
                        Ok(parsed_age) => age = parsed_age,
                        Err(_) => {
                            eprintln!("Error: age must be a number");
                            return;
                        }
                    }
                    i += 2;
                } else {
                    eprintln!("Error: --age requires a value");
                    return;
                }
            }
            "--help" | "-h" => {
                println!("Usage: myprogram --name <NAME> --age <AGE>");
                return;
            }
            _ => {
                eprintln!("Unknown argument: {}", args[i]);
                return;
            }
        }
    }

    if name.is_empty() {
        eprintln!("Error: --name is required");
        return;
    }

    println!("Hello {}, you are {} years old!", name, age);
}
