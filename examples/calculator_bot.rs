// examples/calculator_bot.rs
// A themed calculator with personality - Rusty the Calculator Bot!

use std::io;

fn main() {
    // Bot introduction with personality
    println!("🦀✨ RUSTY THE CALCULATOR BOT ✨🦀");
    println!("=====================================");
    println!("🤖 Rusty: Hey there, friend! I'm Rusty, your friendly Rust-powered calculator!");
    println!("🤖 Rusty: I LOVE solving math problems! Give me expressions like '5 + 3'");
    println!("🤖 Rusty: I can handle +, -, *, and / operations!");
    println!("🤖 Rusty: Type 'quit' when you're done calculating with me!");
    println!();

    let mut calculation_count = 0;

    // Main bot loop
    loop {
        // Friendly prompt
        print!("🤖 Rusty: What's your math problem? > ");
        
        // Flush stdout to ensure prompt appears
        use std::io::Write;
        io::stdout().flush().unwrap();

        // Read user input
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();
                
                // Handle quit with personality
                if input.to_lowercase() == "quit" || input.to_lowercase() == "q" {
                    println!("🤖 Rusty: Aww, leaving so soon? We solved {} problems together!", calculation_count);
                    println!("🤖 Rusty: Thanks for calculating with me! Keep being awesome! 🚀");
                    println!("🤖 Rusty: Remember: Math is everywhere, and so am I! Goodbye! 👋");
                    break;
                }
                
                // Handle empty input
                if input.is_empty() {
                    println!("🤖 Rusty: Hmm, I didn't catch that! Try something like '7 + 2' 🤔");
                    continue;
                }
                
                // Process calculation with bot personality
                match calculate_with_personality(input) {
                    Ok(result) => {
                        calculation_count += 1;
                        show_success_message(input, result, calculation_count);
                    }
                    Err(error) => show_error_message(&error),
                }
            }
            Err(error) => {
                println!("🤖 Rusty: Oops! I had trouble reading that: {}", error);
                break;
            }
        }
        
        println!(); // Spacing for readability
    }
}

/// Calculate with the same logic as main calculator
fn calculate_with_personality(expression: &str) -> Result<f64, String> {
    let parts: Vec<&str> = expression.split_whitespace().collect();
    
    if parts.len() != 3 {
        return Err("I need the format: number operator number (like '5 + 3')!".to_string());
    }
    
    let num1: f64 = match parts[0].parse() {
        Ok(n) => n,
        Err(_) => return Err(format!("'{}' doesn't look like a number to me! 🤨", parts[0])),
    };
    
    let operator = parts[1];
    
    let num2: f64 = match parts[2].parse() {
        Ok(n) => n,
        Err(_) => return Err(format!("'{}' doesn't look like a number either! 🤨", parts[2])),
    };
    
    match operator {
        "+" => Ok(num1 + num2),
        "-" => Ok(num1 - num2),
        "*" => Ok(num1 * num2),
        "/" => {
            if num2 == 0.0 {
                Err("Whoa there! I can't divide by zero - that would break the universe! 🌌".to_string())
            } else {
                Ok(num1 / num2)
            }
        }
        _ => Err(format!("I don't know the '{}' operation! I only know +, -, *, and / 🤷‍♂️", operator)),
    }
}

/// Show success message with personality based on operation and count
fn show_success_message(input: &str, result: f64, count: i32) {
    let operation = if input.contains('+') {
        "addition"
    } else if input.contains('-') {
        "subtraction"
    } else if input.contains('*') {
        "multiplication"
    } else {
        "division"
    };

    // Different responses based on calculation count
    let enthusiasm = match count {
        1 => "🎉 First calculation! ",
        2..=5 => "🔥 Nice! ",
        6..=10 => "🚀 You're on fire! ",
        _ => "🏆 Math master! ",
    };

    // Special responses for certain results
    let special_comment = match result as i32 {
        42 => " (The answer to everything! 🌌)",
        69 => " (Nice! 😎)",
        100 => " (Perfect hundred! 💯)",
        0 => " (Zero - the hero! 🦸‍♂️)",
        _ if result == result.floor() => " (Clean integer! ✨)",
        _ => "",
    };

    println!("🤖 Rusty: {}Ah, {}! The answer is {}{}", 
             enthusiasm, operation, result, special_comment);

    // Occasional encouraging comments
    if count % 3 == 0 {
        let encouragements = [
            "You're getting good at this! 💪",
            "Math is fun when we do it together! 🎈",
            "Keep those calculations coming! 📊",
            "I love solving problems with you! ❤️",
        ];
        let comment = encouragements[count as usize % encouragements.len()];
        println!("🤖 Rusty: {}", comment);
    }
}

/// Show error message with helpful personality
fn show_error_message(error: &str) {
    println!("🤖 Rusty: Oops! {}", error);
    
    // Helpful suggestions based on error type
    if error.contains("format") {
        println!("🤖 Rusty: 💡 Try something like: 8 + 2, or 15 / 3, or 4 * 7!");
    } else if error.contains("number") {
        println!("🤖 Rusty: 💡 Make sure you're using actual numbers, like 5 or 3.14!");
    } else if error.contains("operation") {
        println!("🤖 Rusty: 💡 I only know +, -, *, and / operations!");
    }
}