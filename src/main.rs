use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Welcome to Guessing game");
    println!("You have 5 attempts to guess the secret number between 1 and 100.");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut count = 0;

    loop {
        if count < 5 {
            println!("You have {} attempts remaining", 5 - count);
            count += 1;
        } else {
            println!(
                "You've run out of attempts. The secret number was {}.",
                secret_number
            );
            break;
        }
        println!("please enter ur guess? ");
        let mut guess = String::new();

        //println!("the random number is {secret_number}");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = guess.trim().parse().expect("Please type a number!");
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
