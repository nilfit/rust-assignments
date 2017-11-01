extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1,101);
    let mut tries = 0;
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };
        // only increment tries for valid guesses
        tries = tries + 1;
        println!("You guessed {} on try {}", guess, tries);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                let try_string = match tries {
                    1 => "try",
                    _ => "tries"
                };
                println!("You win after {} {}!", tries, try_string);
                break;
            }
        }
    }
}
