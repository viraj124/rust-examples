// =============================================================================
// CHAPTER 2: PROGRAMMING A GUESSING GAME
// =============================================================================
// This is your first Rust program! It demonstrates:
// 1. Variables and mutability
// 2. Standard input/output
// 3. External crates (rand)
// 4. match expressions
// 5. Loops and control flow
// =============================================================================

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("=== Chapter 2: Guessing Game ===\n");

    println!("Guess the number!");

    // Generate a random number between 1 and 100 (inclusive)
    // rand::thread_rng() creates a random number generator local to the current thread
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Loop until the user guesses correctly
    loop {
        println!("\nPlease input your guess:");

        // Variables are immutable by default
        // `mut` makes this variable mutable so we can modify it
        let mut guess = String::new();

        // Read user input from stdin
        // &mut guess passes a mutable reference to the String
        // .expect() handles the Result type - panics on Err
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Trim whitespace and parse the string to a number
        // This shadows the previous `guess` variable with a new type
        // match handles both Ok and Err variants of Result
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;  // Skip to next iteration of the loop
            }
        };

        println!("You guessed: {guess}");

        // Compare guess to secret_number using cmp()
        // match must handle ALL variants of the Ordering enum
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("🎉 You win!");
                break;  // Exit the loop
            }
        }
    }
}

// =============================================================================
// KEY CONCEPTS FROM THIS CHAPTER
// =============================================================================
//
// VARIABLES:
// - `let` creates an immutable variable
// - `let mut` creates a mutable variable
// - Shadowing: reusing a variable name (often to change types)
//
// I/O:
// - `std::io::stdin()` for reading input
// - `println!()` macro for output
// - {} for variable interpolation in strings
//
// RESULT TYPE:
// - Many operations return Result<T, E>
// - Ok(value) = success
// - Err(error) = failure
// - Use .expect("message") to panic on Err
// - Use match for proper error handling
//
// EXTERNAL CRATES:
// - Add to [dependencies] in Cargo.toml
// - Use with `use crate_name::Type`
//
// CONTROL FLOW:
// - loop {} for infinite loops
// - break to exit a loop
// - continue to skip to next iteration
// - match for pattern matching
// =============================================================================
