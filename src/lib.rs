//! # Bulls and cows
//!
//! `bulls_and_cows` is an implementation of the [Bulls and Cows](https://rosettacode.org/wiki/Bulls_and_cows)
//! coding game.
//!
//! It is just an excuse to learn to program in Rust. The implementation
//! should not only fulfill the task but it must be a complete command line
//! application that includes testing, logging and the documentation
//! you are reading now.
//!
//! The solution is far from optimal, since these are my first steps in this
//! language!
extern crate rand;
use rand::distributions::{Distribution, Uniform};
#[macro_use] extern crate log;
use std::io;
extern crate thiserror;
use thiserror::Error;

/// A type to represent the output of validate_input
type ValidationResult = std::result::Result<u32, ValidationError>;

#[derive(Error, Debug)]
/// Custom error to represent all possible errors that might arise parsing user input
pub enum ValidationError {
    #[error("Parse error on user input")]
    Parse(#[from] std::num::ParseIntError),
    #[error("Input does not respect the rule `{0}`")]
    Invalid(String)
}

/// Creates a random number to be guessed
///
/// It must be a four digit number, without duplication
fn create_random_number() -> u32 {

    let mut chosen: [u32; 4] = [10, 10, 10, 10];
    let mut rng = rand::thread_rng();
    let mut index: usize = 0;
    let die = Uniform::from(0..10);
    loop {
        let number: u32 = die.sample(&mut rng);
        if None == chosen.iter().position(|&x| x == number) {
            chosen[index] = number;
            index += 1;
        }
        if index >= 4 {
            break;
        }
    }
    1000*chosen[0] + 100*chosen[1] + 10*chosen[2] + chosen[3]
}

/// Validate user input against the required format of a guess: four distinct
/// numbers.
///
/// Example, test some inputs:
/// ```
/// # type ValidationResult = std::result::Result<u32, crate::bulls_and_cows::ValidationError>;
/// let result = bulls_and_cows::validate_input("1123").expect_err("will fail");
/// assert_eq!(
///     "Input does not respect the rule `Digits cannot be repeated`",
///     format!("{}", result)
/// );
/// let result = bulls_and_cows::validate_input("1234").unwrap();
/// assert_eq!(1234u32, result);
/// ```
pub fn validate_input(input: &str) -> ValidationResult {

    let mut diff: Vec<u32> = Vec::new();
    let guess = input.parse::<u32>()?;
    for (i, c) in input.chars().enumerate() {
        // do something with character `c` and index `i`
        let digit = c.to_digit(10).unwrap();
        match diff.iter().position(|&x| x == digit) {
            Some(_) => return Err(ValidationError::Invalid(String::from("Digits cannot be repeated"))),
            None => diff.push(digit)
        };
        if i >= 4 {
            return Err(ValidationError::Invalid(String::from("Number has to have 4 digits")));
        }
    }
    Ok(guess)
}

/// Returns the number of bulls (matches at the exact position) and cows
/// (matches) given two string slices
///
/// Example, test some inputs:
/// ```
/// assert_eq!(
///     bulls_and_cows::get_bulls_and_cows("1234", "1234"),
///     (4, 0)
/// );
/// assert_eq!(
///     bulls_and_cows::get_bulls_and_cows("1234", "4321"),
///     (0, 4)
/// );
/// ```
pub fn get_bulls_and_cows(chosen_number: &str, user_guess: &str) -> (u32, u32) {

    let (mut bulls, mut cows) = (0u32, 0u32);
    let guess: String = format!("{:0>4}", user_guess);
    debug!("User guessed '{}'", guess);
    for (index, c) in guess.chars().enumerate() {
        if let Some(i) = chosen_number.chars().position(|x| x == c) {
            if index == i {
                debug!("Got bull match at index '{}'", index);
                bulls += 1;
            } else {
                debug!("Got cow match at index '{}'", index);
                cows += 1;
            }
        };
    }
    (bulls, cows)
}

#[derive(Debug)]
/// Stores the number that the user needs to guess
pub struct ChosenSecret {
    pub number: u32,
    string: String,
}

impl ChosenSecret {
    /// Creates a new number to be guessed
    /// The number is guaranteed to be 4 digit long, with no duplicated digits
    pub fn new() -> ChosenSecret {
        info!("Creating random number");
        let number = create_random_number();
        ChosenSecret {
            number,
            string: number.to_string()
        }
    }
}

/// Main application loop, generates the secret number and allows the user
/// to input guesses, calculating the number of bulls and cows on each attempt.
pub fn run() -> Result<(), String>  {
    println!("I created a random number using four distinct digits...");
    println!("You should guess which one it is");
    let chosen = ChosenSecret::new();
    debug!("The chosen number is {:04}", chosen.number);
    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)
            .expect("Had problems reading user input!");
        // Truncate trailing whitespace
        let len = buffer.trim_end_matches(&['\r', '\n'][..]).len();
        buffer.truncate(len);
        match validate_input(&buffer) {
            Err(validation) => {
                println!("Invalid input: {}\nTry again!", validation);
            },
            Ok(number) => {
                if number == chosen.number {
                    println!("You won! Congratulations!");
                    break;
                }
                let (bulls, cows) = get_bulls_and_cows(&chosen.string, &buffer);
                println!("Nope: you got {} bulls and {} cows. Try again", &bulls, &cows);
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_number() {
        let chosen_number = create_random_number();
        assert!(chosen_number < 10000u32);
    }

    #[test]
    fn test_validation() {

        let err = validate_input("sd41").expect_err("Fails to parse");
        assert_eq!("Parse error on user input", format!("{}", err));
        let err = validate_input("1123").expect_err("Fails to validate");
        assert_eq!("Input does not respect the rule `Digits cannot be repeated`", format!("{}", err));
        let err = validate_input("12345").expect_err("Fails to validate");
        assert_eq!("Input does not respect the rule `Number has to have 4 digits`", format!("{}", err));
        let ok = validate_input("1234").expect("Should not fail");
        assert_eq!(ok, 1234u32);
        let ok = validate_input("234").expect("Should not fail");
        assert_eq!(ok, 234u32);
    }

    #[test]
    fn test_bc_count() {
        assert_eq!((0, 0), get_bulls_and_cows("1234", "5678"));
        assert_eq!((0, 4), get_bulls_and_cows("1234", "4321"));
        assert_eq!((1, 3), get_bulls_and_cows("0123", "312"));
        assert_eq!((4, 0), get_bulls_and_cows("0123", "123"));
    }
}