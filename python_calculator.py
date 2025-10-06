#!/usr/bin/env python3
"""
Python Calculator - Direct comparison to Rust implementation
Demonstrates Python's approach to the same functionality
"""

def calculate(expression):
    """
    Parse and calculate mathematical expressions
    Returns: (result, error) tuple
    """
    try:
        parts = expression.split()
        if len(parts) != 3:
            return None, "Please enter in format: number operator number (e.g., '5 + 3')"
        
        num1 = float(parts[0])
        operator = parts[1]
        num2 = float(parts[2])
        
        if operator == '+':
            return num1 + num2, None
        elif operator == '-':
            return num1 - num2, None
        elif operator == '*':
            return num1 * num2, None
        elif operator == '/':
            if num2 == 0:
                return None, "Cannot divide by zero!"
            return num1 / num2, None
        else:
            return None, f"Unknown operator '{operator}'. Use +, -, *, or /"
    except ValueError as e:
        return None, f"Invalid number: {str(e).split(':')[-1].strip()}"

def main():
    """Main calculator loop"""
    print("🐍 Welcome to Python Calculator!")
    print("Enter expressions like '5 + 3' or type 'quit' to exit.")
    print("Supported operations: +, -, *, /")
    print("Special commands: 'history' to see past calculations, 'clear' to clear history")
    print()
    
    history = []
    
    while True:
        try:
            user_input = input("> ").strip()
            
            if user_input.lower() == 'quit':
                print("Goodbye! 👋")
                break
            elif user_input.lower() == 'history':
                if not history:
                    print("📝 No calculations yet! Start by entering an expression like '5 + 3'")
                else:
                    print(f"📝 Calculation History ({len(history)} entries):")
                    print("=" * 32)
                    for i, entry in enumerate(history, 1):
                        print(f"{i}. {entry}")
                    print("=" * 32)
            elif user_input.lower() == 'clear':
                history.clear()
                print("🗑️ History cleared!")
            else:
                result, error = calculate(user_input)
                if error:
                    print(f"Error: {error}")
                else:
                    print(f"Result: {result}")
                    # Add to history
                    history.append(f"{user_input} = {result}")
            
            print()  # Add blank line for readability
            
        except KeyboardInterrupt:
            print("\nGoodbye! 👋")
            break
        except EOFError:
            print("\nGoodbye! 👋")
            break

if __name__ == "__main__":
    main()