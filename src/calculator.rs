use std::fmt;
use std::io;

// Define an enum called Operation
enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

// Create a function called calculate
fn calculate(op: Operation) -> f64 {
    // Implement the calculate function using pattern matching
    match op {
        Operation::Add(a, b) => a + b,
        Operation::Subtract(a, b) => a - b,
        Operation::Multiply(a, b) => a * b,
        Operation::Divide(a, b) => {
            if b != 0.0 {
                a / b
            } else {
                println!("Error: Division by zero!");
                f64::NAN // Return NaN for division by zero
            }
        }
    }
}

fn main() {
    // Prompt the user to input the first number
    println!("Enter the first number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let first_number: f64 = input.trim().parse().expect("Invalid input");

    // Prompt the user to input the operation
    println!("Enter the operation (+, -, *, /):");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let operation = match input.trim() {
        "+" => Operation::Add(first_number, 0.0), // Initialize second number to 0.0 for unary operations
        "-" => Operation::Subtract(first_number, 0.0),
        "*" => Operation::Multiply(first_number, 0.0),
        "/" => Operation::Divide(first_number, 0.0),
        _ => {
            println!("Invalid operation");
            return;
        }
    };

    // Prompt the user to input the second number
    println!("Enter the second number:");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let second_number: f64 = input.trim().parse().expect("Invalid input");

    // Create an Operation enum instance with the parsed input values
    let operation_instance = match operation {
        Operation::Add(_, _) => Operation::Add(first_number, second_number),
        Operation::Subtract(_, _) => Operation::Subtract(first_number, second_number),
        Operation::Multiply(_, _) => Operation::Multiply(first_number, second_number),
        Operation::Divide(_, _) => Operation::Divide(first_number, second_number),
    };

    // Call the calculate function with the created Operation enum instance
    let result = calculate(operation_instance);

    // Print the result to the console
    println!("Result: {}", result);
}
