// Coding a number guessing game in Rust as in chapter 2 of the book

// Importing necessary crates
use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::{thread, time};

// Main function

fn main() {
    // Generate a random number between 1 and 10
    let secret_number: u8 = rand::rng().random_range(1..=10);

    // Print a welcome message
    println!("Welcome to the number guessing game!");
    println!("Guess the number between 1 and 10 that I am thinking of...");

    // Declare a counter for the number of attempts
    let mut attempts: u8 = 1;

    // Loop until the user guesses the correct answer
    loop {
        // Creating a mutable variable to store the user's guess
        let mut guess = String::new();

        // Prompt the user for a guess
        println!("Attempt {}: Please enter your guess: ", attempts);

        // Read the user's input
        match io::stdin()
            .read_line(&mut guess) {
                Ok(input) => input,
                Err(_) => {
                    println!("Error reading input. Please try again.");
                    continue;
                }
            };
        
        // Convert the input to a number
        let guess: u8 = match guess
            .trim()
            .parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid input. Please enter a number between 1 and 10.");
                    continue;
                }
            };
        
        // Compare the guess with the secret number using cmp::Ordering
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low! Try again."),
            Ordering::Greater => println!("Too high! Try again."),
            Ordering::Equal => {
                println!("Congratulations! You guess the number {} in {} attempts!", secret_number, attempts);
                
                // Introduce a delay before exiting
                let delay = time::Duration::from_secs(3);
                
                println!("Exiting the game in 3 seconds...");
                
                thread::sleep(delay);
                // Exit the loop
                break;
            }
        };
        
        // Increment the attempts counter
        attempts += 1;
        }
    }