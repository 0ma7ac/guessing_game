use rand::Rng; // Import the random number generator from the rand crate
use std::{cmp::Ordering, io}; // Import necessary modules for input/output and comparisons

fn main() {
    // Welcome message for the game
    println!("Welcome to Guessing game");
    println!("You have 5 attempts to guess the secret number between 1 and 100.");

    // Generate a random secret number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // Initialize the attempt counter
    let mut count = 0;

    // Start the main game loop
    loop {
        // Check if attempts are remaining
        if count < 5 {
            println!("You have {} attempts remaining", 5 - count);
            count += 1; // Increment the attempt counter
        } else {
            // Notify the user they are out of attempts and reveal the secret number
            println!(
                "You've run out of attempts. The secret number was {}.",
                secret_number
            );
            break; // Exit the loop
        }

        // Prompt the user to enter their guess
        println!("please enter ur guess? ");
        let mut guess = String::new(); // Create a mutable string to store user input

        // Read the user's input and handle any input errors
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Convert the input string to a u32 number and handle invalid input
        let guess: u32 = guess.trim().parse().expect("Please type a number!");
        println!("You guessed: {}", guess); // Display the user's guess

        // Compare the user's guess with the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"), // If the guess is too small
            Ordering::Greater => println!("Too big!"), // If the guess is too big
            Ordering::Equal => {
                println!("You win!"); // If the guess is correct
                break; // Exit the loop on a correct guess
            }
        }
    }
}
