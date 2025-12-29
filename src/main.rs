// Import for standard input/output
use std::io;
// Import for comparing values
use std::cmp::Ordering;
// Import for random number generation
use rand::Rng;

fn main() {
    // Display welcome message
    println!("Guess the number!");
    // Play the game and display the winning guess
    println!("You guessed: {}", game());
}

// Game function - handles the main game logic and returns the winning guess
fn game() -> u32 {
    // Generate a random secret number between 1 and 100 (inclusive)
    let secret_number = rand::rng().random_range(1..=100);
    println!("The secret number is: {secret_number}");

    // Main game loop - continues until player wins
    loop {
        println!("Please input your guess.");
        
        // Create a mutable string to store the user's input
        let mut guess = String::new();
        
        // Read a line from standard input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Parse the input string as an unsigned 32-bit integer
        // If parsing fails, show an error message and skip to next iteration
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        // Compare the guess with the secret number and provide feedback
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                // Player guessed correctly - display win message and return the guess
                println!("You win!");
                return guess;
            }
        }

    }
}