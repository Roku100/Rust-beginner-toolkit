# Troubleshooting Guide

This document covers common issues you might encounter when learning Rust and how to solve them.

## Common Issues & Fixes

### Issue 1: "rustc is not recognized as an internal or external command"

**What didn't work initially:**
After installing Rust, the rustc and cargo commands weren't recognized in the terminal.

**Error and how you resolved it:**

```
'rustc' is not recognized as an internal or external command, operable program or batch file.
'cargo' is not recognized as an internal or external command, operable program or batch file.
```

**Solution:**
This happens when Rust is installed but not added to your system PATH. Here are the solutions:

**Option 1: Permanent Fix (Recommended)**

1. **Windows GUI Method:**

   - Press `Win + R`, type `sysdm.cpl`, press Enter
   - Click "Environment Variables" button
   - Under "User variables", find and select "Path", click "Edit"
   - Click "New" and add: `C:\Users\[YourUsername]\.cargo\bin`
   - Click "OK" on all dialogs
   - Restart your terminal

2. **PowerShell Method (Run as Administrator):**

```powershell
# Add to user PATH permanently
[Environment]::SetEnvironmentVariable("Path", $env:Path + ";$env:USERPROFILE\.cargo\bin", "User")
```

**Option 2: Restart Terminal (Quick try)**

```bash
# Close and reopen your terminal/PowerShell
# The installer usually sets up PATH but requires a restart
```

**Option 3: Temporary Fix (Current session only)**

```powershell
# For PowerShell - adds Rust to PATH for current session
$env:PATH += ";$env:USERPROFILE\.cargo\bin"

# Verify it works
rustc --version
cargo --version
```

**Verification:**

```bash
# These should work after applying the fix
rustc --version
cargo --version
```

**Expected Output:**

```
rustc 1.XX.X (hash 2024-XX-XX)
cargo 1.XX.X (hash 2024-XX-XX)
```

**Links to StackOverflow, forums, etc:**

- [Rust Installation Guide](https://forge.rust-lang.org/infra/channel-layout.html#rustup)
- [Windows PATH Environment Variable Guide](https://www.architectryan.com/2018/03/17/add-to-the-path-on-windows-10/)

### Issue 2: "cannot borrow `io::stdout()` as mutable"

**What didn't work initially:**
Trying to flush stdout to ensure the prompt appears before user input.

**Error and how you resolved it:**

```
error[E0596]: cannot borrow `io::stdout()` as mutable
```

**Solution:**
Used `io::Write::flush(&mut io::stdout()).unwrap()` instead of `io::stdout().flush().unwrap()` and imported the Write trait with `use std::io::Write`.

### Issue 3: Division by zero handling

**What didn't work initially:**
The calculator would panic when dividing by zero instead of showing a user-friendly error.

**Error and how you resolved it:**
Added explicit check for zero in the division case and returned a custom error message instead of allowing the panic.

## Compilation Errors

### "expected `()`, found `{integer}`"

**Error Message:**

```
error[E0308]: mismatched types
expected `()`, found `{integer}`
```

**Cause:** Function returns a value but is expected to return nothing

**Solution:**

```rust
// Add semicolon to ignore return value:
let result = calculate("5 + 3"); // Remove this line's return value

// Or explicitly return the value:
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // your code
    Ok(())
}
```

### "cannot find function `println` in this scope"

**Error Message:**

```
error[E0425]: cannot find function `println` in this scope
```

**Cause:** Using `println` instead of `println!`

**Solution:**

```rust
// Wrong:
println("Hello, world!");

// Correct:
println!("Hello, world!");
```

### "borrowed value does not live long enough"

**Error Message:**

```
error[E0597]: borrowed value does not live long enough
```

**Cause:** Trying to use a reference to a value that goes out of scope

**Solution:**

```rust
// Problem:
let result = {
    let temp = String::from("hello");
    &temp // temp is dropped here, but we're returning a reference to it
};

// Solution:
let result = String::from("hello"); // Return owned value instead
```

## Runtime Issues

### Calculator not accepting input

**Problem:** Calculator seems to hang after showing the prompt

**Solution:** Make sure to flush stdout before reading input:

```rust
print!("> ");
io::Write::flush(&mut io::stdout()).unwrap();
```

### History not working

**Problem:** History command shows "No calculations yet" even after calculations

**Solution:** Ensure you're adding successful calculations to history:

```rust
if let Ok(result) = calculate(&input) {
    println!("Result: {}", result);
    history.push(format!("{} = {}", input, result));
}
```

## Performance Issues

### Slow compilation times

**Problem:** `cargo build` takes a long time

**Solutions:**

1. Use `cargo check` for faster syntax checking without building
2. Use `cargo build` for debug builds (faster than release)
3. Only use `cargo build --release` for final optimized builds

### Large binary size

**Problem:** Release binary is larger than expected

**Solutions:**

1. Add to `Cargo.toml`:

```toml
[profile.release]
strip = true  # Remove debug symbols
lto = true    # Link-time optimization
codegen-units = 1  # Better optimization
```

## Getting Help

### Official Resources

- [Rust Book](https://doc.rust-lang.org/book/) - Comprehensive guide
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Learn by examples
- [Rust Reference](https://doc.rust-lang.org/reference/) - Language reference

### Community Help

- [Rust Users Forum](https://users.rust-lang.org/) - Ask questions
- [r/rust](https://www.reddit.com/r/rust/) - Reddit community
- [Rust Discord](https://discord.gg/rust-lang) - Real-time chat
- [Stack Overflow](https://stackoverflow.com/questions/tagged/rust) - Q&A

### Debugging Tips

1. **Read error messages carefully** - Rust has excellent error messages
2. **Use `cargo check`** - Faster than full compilation for finding errors
3. **Add `println!` statements** - Debug by printing values
4. **Use `cargo clippy`** - Linting tool for better code
5. **Use `cargo fmt`** - Automatic code formatting

### Best Practices for Beginners

1. **Start small** - Begin with simple programs
2. **Read the compiler errors** - They're usually very helpful
3. **Don't fight the borrow checker** - Learn ownership concepts
4. **Use `cargo doc --open`** - Generate and view documentation
5. **Practice regularly** - Rust concepts take time to internalize

---

If you encounter an issue not covered here, please:

1. Check the official Rust documentation
2. Search the Rust Users Forum
3. Ask on Stack Overflow with the `rust` tag
4. Create an issue in this repository with details about your problem
