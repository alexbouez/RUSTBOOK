//! Rust book, Chapter 2: The Guessing Game
//! 
//! This is an exercise project to learn how to code in Rust 2018.
//! 
//! link : [The Guessing Game](https://doc.rust-lang.org/stable/book/ch02-00-guessing-game-tutorial.html)

use std::io;
use std::cmp::Ordering;

use rand::Rng;

/// Main function of the game
/// 
/// The function picks a random number in 1..=100 and makes the user guess it
/// It asks for user inputs with function get_guess
/// If incorrect, it gives a hint (wether the number is bigger or smaller than guess) then asks again
pub fn main() {
    println!("Guess the secret number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut guess:u32;
    
    loop {
        guess = get_guess();

        match guess.cmp(&secret_number) {
            Ordering::Less      => println!("Too small!"),
            Ordering::Greater   => println!("Too big!"),
            Ordering::Equal     => {
                println!("You win, well played!");
                break;
            }
        }
    }
   
}

/// User input function
/// 
/// Asks for user input and checks format
/// Function loops until correct user input is given
/// 
/// Call example: 
///     let guess:u32 = get.guess();
/// Returns:
///     u32 in 1..=100
fn get_guess() -> u32 {
    println!("\nPlease input your guess:");
    let mut guess = String::new();
    let number:u32;
    let range = 1..101;

    loop {
        guess.clear();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error in Read Line: Failed to read input");

        number = match guess.trim().parse() {
            Ok(num) if range.contains(&num) => num,
            _  => {
                println!("Invalid input!\nPlease input number between 1 and 100:");
                continue;
            }
        };
        break;
    }

    number
}