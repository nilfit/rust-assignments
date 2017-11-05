extern crate rand;

use std::io;
use std::collections::HashMap;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1,101);
    let mut tries = 0;
    let mut guesses = HashMap::new();
    loop {
        println!("Please input your guess.");
        // _s to prevent shadowing so we can add the String to guesses later
        // why not just store the u32?
        let mut guess_s = String::new();
        io::stdin().read_line(&mut guess_s)
            .expect("Failed to read line");
        let guess: u32 = match guess_s.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };
        // only increment tries for valid guesses
        tries = tries + 1;
        guesses.insert(tries, guess_s);
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
    println!("Your last guesses:");
    // convert to something we can sort so that we can get the last 3 guesses
    let mut guesses: Vec<_> = guesses.into_iter().collect();
    guesses.sort();
    for &(n, ref guess) in guesses.iter().rev().take(3) {
        println!("{}\t{}", n, guess.trim());
    }
}
