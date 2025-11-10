//! Bagels, by Prince Muel sam@princemuel.dev
//! A deductive logic game where you must guess a number based on clues.
//! Tags: short, game, puzzle

use core::{fmt, iter};
use std::io::{self, Write};

use rand::seq::IndexedRandom;

const NUM_DIGITS: usize = 3; // (!) Try setting this to 1 or 10.
const MAX_GUESSES: usize = 10; // (!) Try setting this to 1 or 100.

fn main() {
    print_instructions();

    loop {
        play_round();

        if !should_play_again() {
            break;
        }
    }

    println!("Thanks for playing!");
}

fn print_instructions() {
    println!(
        "Bagels, a deductive logic game. By Al Sweigart al@inventwithpython.com

I am thinking of a {number}-digit number with no repeated digits.
Try to guess what it is. Here are some clues:
When I say:         That means:
    Pico            One digit is correct but in the wrong position.
    Fermi           One digit is correct and in the right position.
    Bagels          No digit is correct.

For example, if the secret number was 248 and your guess was 843, the clues would be Fermi Pico.",
        number = NUM_DIGITS
    )
}

fn play_round() {
    let secret = get_secret();
    println!("I have thought up a number.");
    println!(" You have {} guesses to get it.", MAX_GUESSES);

    for guess_num in 1..=MAX_GUESSES {
        let guess = read_valid_guess(guess_num);
        let clue = get_clues(&guess, &secret);

        println!("{}", clue);

        if guess == secret {
            return;
        }
    }

    println!("You ran out of guesses.");
    println!("The answer was {}.", secret);
}

/// Returns a string made up of NUM_DIGITS unique random digits.
fn get_secret() -> String {
    let mut rng = rand::rng();
    b"0123456789"
        .choose_multiple(&mut rng, NUM_DIGITS)
        .map(|b| *b as char)
        .collect()
}

fn read_valid_guess(guess_num: usize) -> String {
    loop {
        print!("Guess #{}: ", guess_num);
        io::stdout().flush().unwrap();

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("Failed to read line");

        let guess = buffer.trim();

        if guess.len() == NUM_DIGITS && guess.chars().all(|c| c.is_ascii_digit()) {
            return guess.into();
        }

        println!("> Invalid guess. Enter exactly {} digits.", NUM_DIGITS);
    }
}

enum Clue {
    Perfect,
    NoMatch,
    Hints { fermis: u8, picos: u8 },
}

impl fmt::Display for Clue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Perfect => write!(f, "You got it!"),
            Self::NoMatch => write!(f, "Bagels"),
            Self::Hints { fermis, picos } => {
                let fermis = iter::repeat_n("Fermi", *fermis as usize);
                let picos = iter::repeat_n("Pico", *picos as usize);

                let mut words = fermis.chain(picos);
                if let Some(first) = words.next() {
                    write!(f, "{}", first)?;

                    for word in words {
                        write!(f, " {}", word)?;
                    }
                }

                Ok(())
            },
        }
    }
}

/// Returns a string with the pico, fermi, bagels clues for a guess and secret
/// number pair.
fn get_clues(guess: &str, secret: &str) -> Clue {
    if guess == secret {
        return Clue::Perfect;
    }

    // Bitset for O(1) digit lookups (digits 0-9 map to bits 0-9)
    let secret_mask = secret
        .as_bytes()
        .iter()
        // flips the relevant bits to one e.g 348 becomes 000000010001100 ?
        // TODO: should there be a difference between 348, 384, 438, 483, 834, 843?
        .fold(0u16, |acc, &b| acc | (1 << (b - b'0')));

    let (mut fermis, mut picos) = (0, 0);

    for (g, s) in guess.bytes().zip(secret.bytes()) {
        if g == s {
            fermis += 1;
        } else if secret_mask & (1 << (g - b'0')) != 0 {
            picos += 1;
        }
    }

    if fermis == 0 && picos == 0 {
        return Clue::NoMatch;
    }

    Clue::Hints { fermis, picos }
}

fn should_play_again() -> bool {
    print!("Do you want to play again? (yes or no)\n> ");
    io::stdout().flush().unwrap();

    let mut buffer = String::with_capacity(4);
    io::stdin().read_line(&mut buffer).expect("Failed to read line");

    buffer.trim().to_lowercase().starts_with('y')
}

#[allow(unused)]
#[deprecated]
fn get_clues_old(guess: &str, secret: &str) -> Clue {
    if guess == secret {
        return Clue::Perfect;
    }

    let (mut fermis, mut picos) = (0, 0);

    for (g, s) in guess.bytes().zip(secret.bytes()) {
        if g == s {
            fermis += 1;
        } else if secret.contains(g as char) {
            picos += 1;
        }
    }

    if fermis == 0 && picos == 0 {
        return Clue::NoMatch;
    }

    Clue::Hints { fermis, picos }
}
