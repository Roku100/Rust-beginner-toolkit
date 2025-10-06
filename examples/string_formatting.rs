// examples/string_formatting.rs
// Advanced string formatting techniques in Rust

fn main() {
    println!("🎨 Rust String Formatting Showcase");
    println!("==================================");
    
    let name = "Dennis";
    let age = 25;
    let score = 87.6543;
    let is_student = true;
    
    // 1. Basic positional arguments
    println!("\n1️⃣ Basic Formatting:");
    println!("Hello, {}! You are {} years old.", name, age);
    
    // 2. Named arguments (most readable)
    println!("\n2️⃣ Named Arguments:");
    println!("Hello, {name}! You scored {score} points.", name = name, score = score);
    
    // 3. Positional with reordering
    println!("\n3️⃣ Reordered Arguments:");
    println!("{1} is {0} years old and scored {2}", age, name, score);
    
    // 4. Number formatting
    println!("\n4️⃣ Number Formatting:");
    println!("Score: {:.2}", score);           // 2 decimal places
    println!("Score: {:.0}", score);           // No decimal places
    println!("Score: {:08.2}", score);         // Pad with zeros, 8 total width
    println!("Age: {:03}", age);               // Pad with zeros, 3 digits
    
    // 5. Alignment and padding
    println!("\n5️⃣ Alignment & Padding:");
    println!("Left:   '{:<10}'", name);        // Left align, 10 chars wide
    println!("Right:  '{:>10}'", name);        // Right align, 10 chars wide
    println!("Center: '{:^10}'", name);        // Center align, 10 chars wide
    println!("Padded: '{:*^10}'", name);       // Center with * padding
    
    // 6. Different number bases
    println!("\n6️⃣ Number Bases:");
    let number = 42;
    println!("Decimal: {}", number);
    println!("Binary:  {:b}", number);
    println!("Octal:   {:o}", number);
    println!("Hex:     {:x}", number);
    println!("HEX:     {:X}", number);
    
    // 7. Debug formatting
    println!("\n7️⃣ Debug Formatting:");
    let data = vec!["apple", "banana", "cherry"];
    println!("Pretty: {:#?}", data);           // Pretty debug print
    println!("Debug:  {:?}", data);            // Regular debug print
    
    // 8. Conditional formatting
    println!("\n8️⃣ Conditional Content:");
    println!("{} is {} student", 
             name, 
             if is_student { "a" } else { "not a" });
    
    // 9. Complex expressions in format strings
    println!("\n9️⃣ Complex Expressions:");
    println!("{name}'s grade: {grade} ({letter})", 
             name = name,
             grade = score,
             letter = match score as i32 {
                 90..=100 => "A",
                 80..=89 => "B", 
                 70..=79 => "C",
                 60..=69 => "D",
                 _ => "F"
             });
    
    // 10. Multi-line strings and formatting
    println!("\n🔟 Multi-line Formatting:");
    let report = format!(
        "📊 Student Report Card
        =====================
        Name: {name}
        Age: {age} years old
        Score: {score:.1}%
        Grade: {grade}
        Status: {status}
        
        📝 Comments: {comment}",
        name = name,
        age = age,
        score = score,
        grade = if score >= 90.0 { "A" } else if score >= 80.0 { "B" } else { "C" },
        status = if is_student { "Active Student" } else { "Alumni" },
        comment = "Excellent progress in Rust programming!"
    );
    println!("{}", report);
    
    // 11. Using format! for building strings
    println!("\n1️⃣1️⃣ Building Strings:");
    let mut achievements = Vec::new();
    achievements.push(format!("🏆 Completed {} exercises", 15));
    achievements.push(format!("⭐ Earned {:.0}% average score", score));
    achievements.push(format!("🎯 Learned {} concepts", 8));
    
    for achievement in achievements {
        println!("   {}", achievement);
    }
    
    // 12. Performance tip: Use println! directly when possible
    println!("\n1️⃣2️⃣ Performance Note:");
    // ✅ Good: Direct printing
    println!("Direct: Hello, {}!", name);
    
    // ❌ Less efficient: Building string then printing
    let message = format!("Indirect: Hello, {}!", name);
    println!("{}", message);
    
    // 13. Escape sequences and special characters
    println!("\n1️⃣3️⃣ Special Characters:");
    println!("Quote: \"Hello, Rust!\"");
    println!("Backslash: \\");
    println!("Tab:\tTabbed text");
    println!("Newline:\nNew line");
    println!("Unicode: \u{1F980}"); // 🦀 crab emoji
    
    // 14. Raw strings (useful for regex, file paths, etc.)
    println!("\n1️⃣4️⃣ Raw Strings:");
    println!(r"Raw string: C:\Users\{}\Documents", name);
    println!(r#"With quotes: "Hello, {}!""#, name);
    
    println!("\n🎉 That's a wrap on Rust string formatting!");
}