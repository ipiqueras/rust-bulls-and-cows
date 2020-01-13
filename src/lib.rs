extern crate rand;
use rand::distributions::{Distribution, Uniform};
#[macro_use] extern crate log;
use std::io::{self, Read};
use std::num::{self};
extern crate thiserror;
use thiserror::Error;
use std::error::Error;


type ValidationResult = std::result::Result<u32, ValidationError>;

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Parse error on user input")]
    Parse(#[from] std::num::ParseIntError),
    #[error("Input does not respect the rule `{0}`")]
    Invalid(String)
}

/// Creates a random number to be guessed
///
/// I must be a four digit number, without duplication
fn create_random_number() -> u32 {

    let mut chosen: [u32; 4] = [10, 10, 10, 10];
    let mut rng = rand::thread_rng();
    let mut index: usize = 0;
    let die = Uniform::from(0..10);
    loop {
        let number: u32 = die.sample(&mut rng);
        match chosen.iter().position(|&x| x == number) {
            None => {
                chosen[index] = number;
                index += 1;
            },
            _ => (),
        }
        if index >= 4 {
            break;
        }
    }
    1000*chosen[0] + 100*chosen[1] + 10*chosen[2] + chosen[3]
}

/// Validate user input against the required format of a guess: four distinct
/// numbers
fn validate_input(input: &str) -> ValidationResult {

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

fn get_bulls_and_cows(chosen_number: &str, user_guess: &str) -> (u32, u32) {
    (0, 0)
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
        let guess = validate_input(&buffer);
        match guess {
            Err(validation) => {
                println!("Validation error!: {}\nTry again", validation);
            },
            Ok(number) => {
                if number == chosen.number {
                    break;
                }
                println!("Try again");
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
}