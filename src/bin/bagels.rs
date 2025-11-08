//! Bagels, by Prince Muel sam@princemuel.dev
//! A deductive logic game where you must guess a number based on clues.
//! Tags: short, game, puzzle

use std::io;

use rand::prelude::SliceRandom;

const NUM_DIGITS: usize = 3; // (!) Try setting this to 1 or 10.
const MAX_GUESSES: usize = 10; // (!) Try setting this to 1 or 100.

fn main() {
    println!(
        "Bagels, a deductive logic game.
By Prince Muel sam@princemuel.dev

I am thinking of a {NUM_DIGITS}-digit number with no repeated digits.
Try to guess what it is. Here are some clues:
When I say:         That means:
    Pico            One digit is correct but in the wrong position.
    Fermi           One digit is correct and in the right position.
    Bagels          No digit is correct.

For example, if the secret number was 248 and your guess was 843, the clues would be Fermi Pico."
    );

    // Main game loop
    loop {
        // This stores the secret number the player needs to guess:
        let secret = get_secret();
        println!("I have thought up a number.");
        println!(" You have {MAX_GUESSES} guesses to get it.");

        let mut guesses = 1;

        while guesses <= MAX_GUESSES {
            let mut guess = String::with_capacity(NUM_DIGITS);

            // Keep looping until they enter a valid guess
            while guess.len() != NUM_DIGITS || guess.parse::<u32>().is_err() {
                println!("Guess #{guesses}:  ");
                guess = io::stdin()
                    .read_line(&mut guess)
                    .unwrap()
                    .to_string()
                    .trim()
                    .to_string();
            }

            let clues = get_clues(&guess, &secret);
            println!("CLUES: {clues}");
            guesses += 1;

            if guess == secret {
                break; // They're correct, so break out of this loop.
            }

            if guesses > MAX_GUESSES {
                println!("You ran out of guesses.");
                println!("The answer was {secret}");
            }
        }

        // Ask the player if they want to play again.
        println!("Do you want to play again? (yes or no)");
        let mut buffer = String::with_capacity(4);
        let input = io::stdin().read_line(&mut buffer).unwrap().to_string();
        if !input.to_lowercase().starts_with("y") {
            break;
        }
    }

    println!("Thanks for playing!");
}

/// Returns a string made up of NUM_DIGITS unique random digits.
fn get_secret() -> String {
    let mut rng = &mut rand::thread_rng();
    let sample = "0123456789".as_bytes();
    let secret = sample.choose_multiple(&mut rng, NUM_DIGITS).copied().collect();
    unsafe { String::from_utf8_unchecked(secret) }
}

/// Returns a string with the pico, fermi, bagels clues for a guess and secret
/// number pair.
fn get_clues(guess: &str, secret: &str) -> String {
    if guess == secret {
        return "You got it!".to_owned();
    }

    let (guess, secret) = (guess.as_bytes(), secret.as_bytes());

    let mut clues = vec![];

    for i in 0..guess.len() {
        if guess[i] == secret[i] {
            // A correct digit is in the correct place.
            clues.push("Fermi".to_string());
        } else if secret.contains(&guess[i]) {
            // A correct digit is in the incorrect place.
            clues.push("Pico".to_string());
        }
    }

    if clues.is_empty() {
        return "Bagels".to_string(); // There are no correct digits at all.
    }

    // Sort the clues into alphabetical order so their original order doesn't give
    // information away
    clues.sort_unstable();
    // Make a single string from the list of string clues.
    clues.join(" ")
}
