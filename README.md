# Guessing Game 🎲

Welcome to the **Guessing Game**, a beginner-friendly Rust project! This simple yet engaging game challenges you to guess a randomly generated number between 1 and 100 within 5 attempts. Test your luck and logical reasoning!

---

## 🌟 Features
- **Random Secret Number:** A new secret number is generated each time you run the game.
- **Hints Provided:** Get feedback on whether your guess is too low or too high.
- **Limited Attempts:** You have only 5 attempts to guess correctly.
- **Interactive Gameplay:** User-friendly prompts and responses.

---

## 🚀 Getting Started

### Prerequisites
- Rust installed on your system. If you don’t have Rust, [install it here](https://www.rust-lang.org/tools/install).

### Cloning the Repository
1. Clone the repository using Git:
    ```bash
    git clone https://github.com/Oma7ac/guessing_game.git
    cd guessing_game
    ```

2. Navigate to the project folder:
    ```bash
    cd guessing_game
    ```

---

## 🛠️ Running the Game

To run the game, use the following commands:
1. Compile and run the program:
    ```bash
    cargo run
    ```

2. Follow the on-screen instructions to play!

---

## 📂 Project Structure

Here’s a breakdown of the project files:
- **src/main.rs:** Contains the game logic.
- **Cargo.toml:** Defines the dependencies for the project.
- **Cargo.lock:** Tracks the exact versions of dependencies used.

---

## 📖 How to Play
1. Run the program using the `cargo run` command.
2. The game will display a welcome message and instructions.
3. Enter your guesses when prompted.
4. Get feedback:
   - **"Too small!"** if your guess is less than the secret number.
   - **"Too big!"** if your guess is greater than the secret number.
   - **"You win!"** if your guess matches the secret number.
5. Use your 5 attempts wisely! If you don’t guess correctly, the secret number will be revealed.

---

## 🖥️ Example Gameplay

```plaintext
Welcome to Guessing game
You have 5 attempts to guess the secret number between 1 and 100.
You have 5 attempts remaining
please enter ur guess? 
50
You guessed: 50
Too small!
You have 4 attempts remaining
please enter ur guess? 
75
You guessed: 75
Too big!
...
You've run out of attempts. The secret number was 42.
