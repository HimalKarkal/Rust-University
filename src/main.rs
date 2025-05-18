// Listing the first 10000 prime numbers
// Imports
use std::time::Instant;

fn primes_to_100000() {
    // Starting the timer
    let start = Instant::now();

    let mut primes_vector: Vec<u32> = vec![2];
    let mut current_number: u32 = 3;

    while primes_vector.len() < 100000 {
        
        let mut is_prime: bool = true; // Creating a boolean variable to check if the current number is prime
        for prime in &primes_vector {
            if &current_number % prime == 0 {
                is_prime = false;
                break; // If the current number is divisible by any prime, it is not prime
            }
        }
        if is_prime {
            primes_vector.push(current_number);
        }

        current_number += 1;
    }

    println!("The 100,000th prime number is: {}.", primes_vector.get(99999).unwrap());

    // Ending the timer
    let duration = start.elapsed();
    println!("Time taken to find the first 100,000 prime numbers is: {} ms.", duration.as_millis())
}

fn main(){
    //Calling the function to find the first 100,000 prime numbers
    primes_to_100000();
    }
