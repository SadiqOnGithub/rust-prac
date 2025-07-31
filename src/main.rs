use std::error::Error;
use std::fmt;

// Custom error type
#[derive(Debug)]
struct CustomError {
    message: String,
}

impl CustomError {
    fn new(msg: &str) -> Self {
        CustomError {
            message: msg.to_string(),
        }
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Custom error: {}", self.message)
    }
}

impl Error for CustomError {}

// Function that can return different types of errors
fn demonstrate_errors(scenario: i32) -> Result<String, Box<dyn Error>> {
    match scenario {
        1 => {
            // Return a custom error
            Err(Box::new(CustomError::new("Something went wrong")))
        }
        2 => {
            // Try to read a file (might return io::Error)
            let content = std::fs::read_to_string("nonexistent.txt")?;
            Ok(content)
        }
        3 => {
            // Try to parse a string (might return ParseIntError)
            let number: i32 = "not_a_number".parse()?;
            Ok(format!("Number: {}", number))
        }
        _ => Ok("Success!".to_string()),
    }
}

fn main() {
    // All of these return Box<dyn Error>, but different concrete error types

    println!("Scenario 1:");
    match demonstrate_errors(1) {
        Ok(result) => println!("Success: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    println!("\nScenario 2:");
    match demonstrate_errors(2) {
        Ok(result) => println!("Success: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    println!("\nScenario 3:");
    match demonstrate_errors(3) {
        Ok(result) => println!("Success: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    println!("\nScenario 4:");
    match demonstrate_errors(4) {
        Ok(result) => println!("Success: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
