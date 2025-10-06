// examples/variables.rs
// Learning about variables, data types, and mutability in Rust

fn main() {
    println!("🦀 Learning Rust Variables and Data Types!");
    println!("==========================================");
    
    // Immutable variables (default in Rust)
    let x = 5;
    let name = "Calculator";
    println!("Immutable variable x: {}", x);
    println!("Immutable string name: {}", name);
    
    // Mutable variables (need 'mut' keyword)
    let mut counter = 0;
    println!("Counter starts at: {}", counter);
    counter = counter + 1;
    println!("Counter after increment: {}", counter);
    
    // Different number types (important for calculator)
    let integer: i32 = 42;           // 32-bit signed integer
    let float: f64 = 3.14159;        // 64-bit floating point
    let result = integer as f64 + float; // Type conversion
    
    println!("Integer: {}", integer);
    println!("Float: {}", float);
    println!("Mixed calculation: {} + {} = {}", integer, float, result);
    
    // Strings vs string slices
    let string_slice = "Hello";      // &str (string slice)
    let owned_string = String::from("World"); // String (owned)
    let combined = format!("{} {}", string_slice, owned_string);
    
    println!("String slice: {}", string_slice);
    println!("Owned string: {}", owned_string);
    println!("Combined: {}", combined);
    
    // Boolean type
    let is_learning_rust = true;
    let is_difficult = false;
    
    println!("Am I learning Rust? {}", is_learning_rust);
    println!("Is Rust difficult? {}", is_difficult);
}