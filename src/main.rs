// src/main.rs
// CLI Calculator - Main Application
// This is our capstone project: a beginner-friendly calculator in Rust

use std::io;

fn main() {
    println!("🦀 Welcome to Rust Calculator!");
    println!("Enter expressions like '5 + 3' or type 'quit' to exit.");
    println!("Supported operations: +, -, *, /");
    println!("Special commands: 'history' to see past calculations, 'clear' to clear history");
    println!();

    // Store calculation history
    let mut history: Vec<String> = Vec::new();

    // Main calculator loop
    loop {
        // Display prompt
        print!("> ");
        
        // Flush stdout to ensure prompt appears immediately
        use std::io::Write;
        io::stdout().flush().unwrap();

        // Read user input
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim(); // Remove whitespace and newline
                
                // Check if user wants to quit
                if input.to_lowercase() == "quit" || input.to_lowercase() == "q" {
                    println!("Thanks for using Rust Calculator! You performed {} calculations.", history.len());
                    println!("Goodbye! 👋");
                    break;
                }
                
                // Handle special commands
                if input.to_lowercase() == "history" {
                    show_history(&history);
                    continue;
                }
                
                if input.to_lowercase() == "clear" {
                    history.clear();
                    println!("History cleared! 🧹");
                    continue;
                }
                
                // Skip empty input
                if input.is_empty() {
                    continue;
                }
                
                // Process the calculation
                match calculate(input) {
                    Ok(result) => {
                        println!("Result: {}", result);
                        // Add to history
                        history.push(format!("{} = {}", input, result));
                    }
                    Err(error) => println!("Error: {}", error),
                }
            }
            Err(error) => {
                println!("Error reading input: {}", error);
                break;
            }
        }
        
        println!(); // Add blank line for readability
    }
}

/// Parse and calculate mathematical expressions
/// Supports format: "number operator number" (e.g., "5 + 3")
fn calculate(expression: &str) -> Result<f64, String> {
    // Split the input into parts
    let parts: Vec<&str> = expression.split_whitespace().collect();
    
    // Validate we have exactly 3 parts
    if parts.len() != 3 {
        return Err("Please enter in format: number operator number (e.g., '5 + 3')".to_string());
    }
    
    // Parse the first number
    let num1: f64 = match parts[0].parse() {
        Ok(n) => n,
        Err(_) => return Err(format!("'{}' is not a valid number", parts[0])),
    };
    
    // Get the operator
    let operator = parts[1];
    
    // Parse the second number
    let num2: f64 = match parts[2].parse() {
        Ok(n) => n,
        Err(_) => return Err(format!("'{}' is not a valid number", parts[2])),
    };
    
    // Perform the calculation based on operator
    match operator {
        "+" => Ok(num1 + num2),
        "-" => Ok(num1 - num2),
        "*" => Ok(num1 * num2),
        "/" => {
            if num2 == 0.0 {
                Err("Cannot divide by zero!".to_string())
            } else {
                Ok(num1 / num2)
            }
        }
        _ => Err(format!("Unknown operator '{}'. Use +, -, *, or /", operator)),
    }
}

/// Display calculation history
fn show_history(history: &Vec<String>) {
    if history.is_empty() {
        println!("📝 No calculations yet! Start by entering an expression like '5 + 3'");
    } else {
        println!("📝 Calculation History ({} entries):", history.len());
        println!("================================");
        for (i, calculation) in history.iter().enumerate() {
            println!("{}. {}", i + 1, calculation);
        }
        println!("================================");
    }
}

// Unit tests for our calculator functions
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        assert_eq!(calculate("5 + 3").unwrap(), 8.0);
    }

    #[test]
    fn test_subtraction() {
        assert_eq!(calculate("10 - 4").unwrap(), 6.0);
    }

    #[test]
    fn test_multiplication() {
        assert_eq!(calculate("6 * 7").unwrap(), 42.0);
    }

    #[test]
    fn test_division() {
        assert_eq!(calculate("15 / 3").unwrap(), 5.0);
    }

    #[test]
    fn test_division_by_zero() {
        assert!(calculate("5 / 0").is_err());
    }

    #[test]
    fn test_invalid_number() {
        assert!(calculate("hello + 3").is_err());
    }

    #[test]
    fn test_invalid_operator() {
        assert!(calculate("5 % 3").is_err());
    }
}
