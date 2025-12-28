# Guessing Game

A simple number guessing game written in Rust where the computer picks a random number between 1 and 100, and you try to guess it.

## Game Overview

When you run the game:
1. The computer randomly generates a secret number between 1 and 100
2. You're prompted to enter your guess
3. The game tells you if your guess is too small, too big, or correct
4. Keep guessing until you find the secret number and win!

## Code Structure

The game is built with Rust and uses the following key components:

- **`rand::rng().random_range(1..=100)`** - Generates a random secret number
- **`io::stdin().read_line()`** - Reads user input from the command line
- **`String::parse()`** - Converts the user's input from text to a number
- **`Ordering` enum** - Compares the guess with the secret number (Less, Greater, or Equal)
- **Loop** - Continues until the player guesses correctly

### Dependencies

- **rand 0.9.2** - Used to generate random numbers

## How to Run

### Prerequisites

- Rust and Cargo installed ([Install Rust](https://www.rust-lang.org/tools/install))

### Running the Game

```bash
cargo run
```

This will compile and run the game. Once running, enter numbers at the prompt to guess the secret number.

### Building for Release

For an optimized version:

```bash
cargo build --release
```

The compiled binary will be in `target/release/guessing_game`.

## Example Gameplay

```
Guess the number!
Please input your guess.
50
Too big!
You guessed: 50
Please input your guess.
25
Too small!
You guessed: 25
Please input your guess.
37
Too big!
You guessed: 37
Please input your guess.
31
You win!
```

## License

This is a learning project.
