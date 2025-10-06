// examples/user_input.rs
// Learning how to handle user input in Rust

use std::io;

fn main() {
    println!("🦀 Learning User Input in Rust!");
    println!("================================");
    
    // Basic input example
    println!("What's your name?");
    let mut name = String::new();
    
    match io::stdin().read_line(&mut name) {
        Ok(_) => {
            let name = name.trim(); // Remove newline
            println!("Hello, {}!", name);
        }
        Err(error) => println!("Error reading input: {}", error),
    }
    
    // Number input example
    println!("\nEnter a number:");
    let mut input = String::new();
    
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let input = input.trim();
            match input.parse::<f64>() {
                Ok(number) => {
                    println!("You entered: {}", number);
                    println!("Double that number: {}", number * 2.0);
                }
                Err(_) => println!("'{}' is not a valid number!", input),
            }
        }
        Err(error) => println!("Error reading input: {}", error),
    }
    
    // Simple calculator input simulation
    println!("\nSimple calculation (format: number operator number):");
    println!("Example: 5 + 3");
    
    let mut calc_input = String::new();
    match io::stdin().read_line(&mut calc_input) {
        Ok(_) => {
            let calc_input = calc_input.trim();
            match parse_calculation(calc_input) {
                Ok((num1, operator, num2)) => {
                    let result = perform_calculation(num1, operator, num2);
                    match result {
                        Ok(answer) => println!("Result: {}", answer),
                        Err(error) => println!("Calculation error: {}", error),
                    }
                }
                Err(error) => println!("Parse error: {}", error),
            }
        }
        Err(error) => println!("Input error: {}", error),
    }
}

// Function to parse "5 + 3" format
fn parse_calculation(input: &str) -> Result<(f64, char, f64), String> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    
    if parts.len() != 3 {
        return Err("Please use format: number operator number".to_string());
    }
    
    let num1: f64 = parts[0].parse()
        .map_err(|_| format!("'{}' is not a valid number", parts[0]))?;
    
    let operator = parts[1].chars().next()
        .ok_or("Invalid operator")?;
    
    let num2: f64 = parts[2].parse()
        .map_err(|_| format!("'{}' is not a valid number", parts[2]))?;
    
    Ok((num1, operator, num2))
}

// Function to perform the calculation
fn perform_calculation(num1: f64, operator: char, num2: f64) -> Result<f64, String> {
    match operator {
        '+' => Ok(num1 + num2),
        '-' => Ok(num1 - num2),
        '*' => Ok(num1 * num2),
        '/' => {
            if num2 == 0.0 {
                Err("Cannot divide by zero!".to_string())
            } else {
                Ok(num1 / num2)
            }
        }
        _ => Err(format!("Unknown operator '{}'", operator)),
    }
}