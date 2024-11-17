// examples/number_validator.rs

use std::io::{self, Write};
use is_number::is_number;

fn main() {
    println!("Number Validator");
    println!("---------------");
    println!("Type 'quit' to exit\n");

    loop {
        print!("Enter a value to check if it's a number: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();

        if input.eq_ignore_ascii_case("quit") {
            println!("Goodbye!");
            break;
        }

        if is_number(input) {
            println!("✓ \"{}\" is a valid number!", input);
            
            // Show what type of number it might be
            if input.contains(['e', 'E']) {
                println!("  Type: Scientific notation");
            } else if input.contains('.') {
                println!("  Type: Decimal number");
            } else {
                println!("  Type: Integer");
            }
        } else {
            println!("✗ \"{}\" is not a valid number.", input);
        }
        println!();
    }
}