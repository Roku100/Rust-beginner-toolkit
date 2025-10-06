// examples/hello_world.rs
// A simple Hello World example to demonstrate basic Rust syntax

fn main() {
    // Print a greeting to the console
    println!("Hello, World from Rust! 🦀");
    println!("Welcome to your Rust learning journey!");
    println!();
    
    // Demonstrate different variable types and string formatting
    let learner_name = "Future Rustacean";
    let days_learning = 1;
    let hours_today = 2.5;
    let is_enjoying = true;
    let favorite_emoji = '🦀';
    
    // Method 1: Named parameters (most readable)
    println!("👋 Hi! I'm {name} and I've been learning Rust for {days} day(s)!", 
             name = learner_name, 
             days = days_learning);
    
    // Method 2: Positional parameters with different content
    println!("📚 Today I spent {:.1} hours coding and I'm {} it!", 
             hours_today, 
             if is_enjoying { "loving" } else { "struggling with" });
    
    // Method 3: Using format! macro for complex strings
    let status_message = format!(
        "🎯 Progress Report:\n   • Name: {}\n   • Days learning: {}\n   • Favorite Rust symbol: {}\n   • Motivation level: {}",
        learner_name,
        days_learning,
        favorite_emoji,
        if days_learning < 7 { "🔥 On fire!" } else { "🚀 Experienced!" }
    );
    println!("{}", status_message);
    
    println!();
    
    // Demonstrate basic math with better formatting
    let x = 15;
    let y = 7;
    let sum = x + y;
    let difference = x - y;
    let product = x * y;
    let quotient = x as f64 / y as f64; // Convert to float for division
    
    println!("🧮 Math Practice:");
    println!("   {} + {} = {}", x, y, sum);
    println!("   {} - {} = {}", x, y, difference);
    println!("   {} × {} = {}", x, y, product);
    println!("   {} ÷ {} = {:.2}", x, y, quotient);
    
    // Demonstrate string methods and more complex formatting
    let rust_facts = vec![
        "Rust prevents memory leaks",
        "Rust has zero-cost abstractions", 
        "Rust is blazingly fast",
        "Rust has an amazing community"
    ];
    
    println!();
    println!("💡 Cool Rust Facts:");
    for (index, fact) in rust_facts.iter().enumerate() {
        println!("   {}. {}", index + 1, fact);
    }
    
    // Show different ways to handle pluralization
    let items_learned = 5;
    println!();
    println!("🎉 I've learned {} concept{} so far!", 
             items_learned,
             if items_learned == 1 { "" } else { "s" });
    
    // Demonstrate conditional expressions
    let encouragement = match days_learning {
        1 => "Great start! Keep going! 🌱",
        2..=7 => "You're building momentum! 💪",
        8..=30 => "Looking good! You're getting the hang of it! 🎯",
        _ => "Wow, you're becoming a Rust expert! 🏆"
    };
    
    println!("📈 {}", encouragement);
}