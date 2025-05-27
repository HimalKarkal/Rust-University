// Coding a simple calculator in Rust
// It takes two numbers and a mathematical operator as input and outputs the result.
// Importing necessary modules
use std::io;
use std::{thread, time};

// Main function
fn main() {
    // DECLARING VARIABLES
    let mut no_1: String = String::new();
    let mut no_2: String = String::new();
    let mut operator: String = String::new();

    // PROMPTING USER FOR INPUT
    println!("RUST CALCULATOR \n"); // Title

    // Taking first number input
    println!("Enter first number (float): ");
    io::stdin()
        .read_line(&mut no_1)
        .expect("Failed to read line");
    let no_1: f64 = no_1.trim().parse().expect("Invalid number!");

    // Taking operator input
    println!("Enter operator (+,-,*,/): ");
    io::stdin()
        .read_line(&mut operator)
        .expect("Failed to read line");
    let operator: char = operator.trim().parse().expect("Invalid operator!");

    // Taking first number input
    println!("Enter second number (float): ");
    io::stdin()
        .read_line(&mut no_2)
        .expect("Failed to read line");
    let no_2: f64 = no_2.trim().parse().expect("Invalid number!");

    // INSTANTIATING OPERATOR ENUM
    let operator: Operation = match operator {
        '+' => Operation::Add,
        '-' => Operation::Subtract,
        '*' => Operation::Multiply,
        '/' => Operation::Divide,
        _ => {
            println!("Invalid operator! Please use +, -, *, or /");
            return;
        }
    };
    // CREATING CALCULATOR STRUCT
    let calculator = Calculator {
        no_1,
        no_2,
        operator
    };

    //CALLING THE CALCULATE METHOD
    calculator.calculate();
    println!("\nThank you for using the Rust Calculator!");
    println!("Exiting in 3 seconds...");

    // Adding a delay before exiting
    let delay:u64 = 2;
    let delay:time::Duration = time::Duration::from_secs(delay);
    thread::sleep(delay);

}

enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

struct Calculator {
    no_1: f64,
    no_2: f64,
    operator: Operation,
}

impl Calculator {
    fn calculate(&self) {
        match self.operator {
            Operation::Add => {
                println!("\nThe sum of {} and {} is: {}", self.no_1, self.no_2, self.no_1 + self.no_2);
            },
            Operation::Subtract => {
                println!("\nThe difference of {} and {} is: {}", self.no_1, self.no_2, self.no_1 - self.no_2);
            },
            Operation::Multiply=> {
                println!("\nThe product of {} and {} is: {}", self.no_1, self.no_2, self.no_1 * self.no_2);
            },
            Operation::Divide => {
                match self.no_2 {
                    0.0 => {
                        println!("\nCannot divide by zero!");
                    },
                    _ => {
                        println!("\nThe quotient of {} and {} is: {}", self.no_1, self.no_2, self.no_1 / self.no_2);
                    },
                }

            },

        }
    }
}