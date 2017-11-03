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
        let guess = match read_u32() {
            Ok(num) => num,
            Err(e) => {
                println!("{}", e);
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

fn read_u32() -> Result<u32, String> {
    let mut s = String::new();
    match io::stdin().read_line(&mut s) {
        Err(e) => return Err(format!("{:?}", e)),
        _ => ()
    };
    match s.trim().parse() {
        Ok(num) => Ok(num),
        Err(e) => Err(format!("in parsing u32, {:?}", e))
    }
}
