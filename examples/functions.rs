// examples/functions.rs
// Learning about functions in Rust - essential for our calculator

fn main() {
    println!("🦀 Learning Rust Functions!");
    println!("===========================");
    
    // Call functions with different parameters
    greet("Rust Learner");
    
    let sum = add_numbers(5, 3);
    println!("5 + 3 = {}", sum);
    
    let difference = subtract(10.5, 3.2);
    println!("10.5 - 3.2 = {}", difference);
    
    let product = multiply(4, 7);
    println!("4 * 7 = {}", product);
    
    // Function that returns a Result (for error handling)
    match divide(10.0, 2.0) {
        Ok(result) => println!("10.0 / 2.0 = {}", result),
        Err(error) => println!("Error: {}", error),
    }
    
    // Test division by zero
    match divide(5.0, 0.0) {
        Ok(result) => println!("5.0 / 0.0 = {}", result),
        Err(error) => println!("Error: {}", error),
    }
    
    // Function that validates input
    let test_inputs = vec!["5", "hello", "3.14", ""];
    for input in test_inputs {
        match parse_number(input) {
            Ok(num) => println!("'{}' parsed to: {}", input, num),
            Err(error) => println!("'{}' failed to parse: {}", input, error),
        }
    }
}

// Function with no return value
fn greet(name: &str) {
    println!("Hello, {}! Welcome to Rust functions!", name);
}

// Function that returns a value
fn add_numbers(a: i32, b: i32) -> i32 {
    a + b  // No semicolon = return value
}

// Function with floating point numbers
fn subtract(a: f64, b: f64) -> f64 {
    return a - b;  // Explicit return (also valid)
}

// Another arithmetic function
fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

// Function that returns Result for error handling
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Cannot divide by zero!".to_string())
    } else {
        Ok(a / b)
    }
}

// Function to parse string to number (useful for calculator input)
fn parse_number(input: &str) -> Result<f64, String> {
    match input.trim().parse::<f64>() {
        Ok(number) => Ok(number),
        Err(_) => Err(format!("'{}' is not a valid number", input)),
    }
}