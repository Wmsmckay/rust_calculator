use std::io;

fn main() {
    println!("Welcome to the Rust Calculator!");

    loop {
        println!("Please enter your calculation (e.g. 2 + 2):");

        // Read input from user
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // Split input into operands and operator
        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        // Ensure input has exactly three parts (two operands and an operator)
        if parts.len() != 3 {
            println!("Invalid input. Please try again.");
            continue;
        }

        // Parse operands as floats
        let a: f64 = match parts[0].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };
        let b: f64 = match parts[2].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        // Perform calculation based on operator
        let result = match parts[1] {
            "+" => a + b,
            "-" => a - b,
            "*" => a * b,
            "/" => {
                if b == 0.0 {
                    println!("Cannot divide by zero.");
                    continue;
                }
                a / b
            }
            _ => {
                println!("Invalid operator. Please use +, -, *, or /.");
                continue;
            }
        };

        // Print result
        println!("Result: {}", result);
    }
}