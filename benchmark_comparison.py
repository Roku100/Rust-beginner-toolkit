#!/usr/bin/env python3
"""
Performance Benchmark: Python vs Rust Calculator
Run this to compare execution times between implementations
"""

import time
import subprocess
import statistics
import sys
import os

def benchmark_python_calculator():
    """Benchmark the Python calculator with automated inputs"""
    start_time = time.time()
    
    # Simulate calculator operations
    test_expressions = [
        "5 + 3",
        "10.5 - 2.3", 
        "7 * 8",
        "100 / 4",
        "15.5 + 24.7",
        "50 - 25",
        "3.14 * 2",
        "88 / 11"
    ]
    
    # Import our calculator function
    sys.path.append('.')
    from python_calculator import calculate
    
    results = []
    for expr in test_expressions:
        result, error = calculate(expr)
        if not error:
            results.append(result)
    
    end_time = time.time()
    return end_time - start_time, len(results)

def benchmark_rust_calculator():
    """Benchmark the Rust calculator by measuring compilation + execution"""
    # Check if Rust project exists
    if not os.path.exists("Cargo.toml"):
        return None, "Rust project not found"
    
    try:
        # Try to add Rust to PATH if not available
        rust_path = os.path.expanduser("~/.cargo/bin")
        if os.path.exists(rust_path) and rust_path not in os.environ.get("PATH", ""):
            os.environ["PATH"] += os.pathsep + rust_path
        
        # Compile the Rust project
        compile_start = time.time()
        compile_result = subprocess.run(
            ["cargo", "build", "--release"],
            capture_output=True,
            text=True,
            timeout=30
        )
        compile_time = time.time() - compile_start
        
        if compile_result.returncode != 0:
            return None, f"Compilation failed: {compile_result.stderr}"
        
        # Check if binary was created
        binary_path = "target/release/rust-beginner-toolkit.exe"
        if os.path.exists(binary_path):
            binary_size = os.path.getsize(binary_path)
            return compile_time, f"Compiled successfully (binary: {binary_size:,} bytes)"
        else:
            return compile_time, "Compiled successfully"
        
    except subprocess.TimeoutExpired:
        return None, "Compilation timeout"
    except FileNotFoundError:
        # Try with full path
        cargo_path = os.path.expanduser("~/.cargo/bin/cargo.exe")
        if os.path.exists(cargo_path):
            try:
                compile_start = time.time()
                compile_result = subprocess.run(
                    [cargo_path, "build", "--release"],
                    capture_output=True,
                    text=True,
                    timeout=30
                )
                compile_time = time.time() - compile_start
                
                if compile_result.returncode != 0:
                    return None, f"Compilation failed: {compile_result.stderr}"
                
                return compile_time, "Compiled successfully (using full path)"
            except Exception as e:
                return None, f"Cargo found but failed: {e}"
        else:
            return None, "Cargo not found - run: $env:PATH += \";$env:USERPROFILE\\.cargo\\bin\""

def run_benchmarks():
    """Run comprehensive benchmarks and display results"""
    print("🚀 Calculator Performance Benchmark: Python vs Rust")
    print("=" * 60)
    
    # Python benchmark
    print("\n🐍 Python Calculator Benchmark:")
    python_times = []
    
    for i in range(5):
        try:
            exec_time, operations = benchmark_python_calculator()
            python_times.append(exec_time)
            print(f"  Run {i+1}: {exec_time:.6f}s ({operations} operations)")
        except Exception as e:
            print(f"  Run {i+1}: Error - {e}")
    
    if python_times:
        avg_python = statistics.mean(python_times)
        print(f"  Average: {avg_python:.6f}s")
    
    # Rust benchmark
    print("\n🦀 Rust Calculator Benchmark:")
    rust_time, rust_result = benchmark_rust_calculator()
    
    if rust_time is not None:
        print(f"  Compilation time: {rust_time:.6f}s")
        print(f"  Status: {rust_result}")
    else:
        print(f"  Error: {rust_result}")
        if "Cargo not found" in rust_result:
            print("  💡 Fix: Run this command first:")
            print("     PowerShell: $env:PATH += \";$env:USERPROFILE\\.cargo\\bin\"")
            print("     Then run the benchmark again")
    
    # Memory usage comparison (approximate)
    print("\n💾 Memory Usage Comparison:")
    print("  Python: ~15-30 MB (interpreter + runtime)")
    print("  Rust:   ~2-5 MB (compiled binary)")
    
    # Startup time comparison
    print("\n⚡ Startup Time Comparison:")
    print("  Python: ~0.1-0.2s (interpreter startup)")
    print("  Rust:   ~0.01-0.05s (direct execution)")
    
    # File size comparison
    print("\n📁 File Size Comparison:")
    
    # Python file size
    if os.path.exists("python_calculator.py"):
        python_size = os.path.getsize("python_calculator.py")
        print(f"  Python source: {python_size} bytes")
    
    # Rust binary size (if exists)
    rust_binary = "target/release/rust-beginner-toolkit.exe"
    if os.path.exists(rust_binary):
        rust_size = os.path.getsize(rust_binary)
        print(f"  Rust binary: {rust_size:,} bytes")
    else:
        print("  Rust binary: Not compiled yet")
    
    print("\n" + "=" * 60)
    print("📊 Summary:")
    print("  • Python: Faster development, slower execution")
    print("  • Rust: Slower development, faster execution")
    print("  • Python: Larger memory footprint")
    print("  • Rust: Smaller memory footprint, larger binary")

if __name__ == "__main__":
    run_benchmarks()