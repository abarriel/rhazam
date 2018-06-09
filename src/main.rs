extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);
    loop {
        let (err, guess) = get_guess();
        if err {
            continue;
        }

        if test_answer(guess, secret_number) {
            break;
        }
    }
}

fn get_guess() -> (bool, i32) {
    println!("Enter your guess:");
    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    
    let guess: (bool, i32)  = match guess.trim().parse() {
        Ok(num) => (false, num),
        Err(_) => {
            println!("Please enter a number");
            (true, 0)
        },
    };
    return guess;
}

fn test_answer(guess: i32, secret_number: i32) -> bool {
    match guess.cmp(&secret_number) {
        Ordering::Less => {
            println!("Too small!");
            false
        },
        Ordering::Greater => {
            println!("Too big!");
            false
        },
        Ordering::Equal => {
            println!("You win!");
            true
        },
    }
}