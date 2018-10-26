// create rand as external dependency -  same as 'use rand'
extern crate rand;
use std::io;
use std::cmp::Ordering;
// adds Rng trait for methods
use rand::Rng;

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got: {}", value);
        }
        Guess {
            value
        }
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}


fn main() {
    println!("Guess the number!");

    // random number
    let secret_number = rand::thread_rng().gen_range(1,101);

    //println!("The number is: {}", secret_number);
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        // read in guess
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line.");

        // convert guess string to int
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess < 1 || guess > 100 {
            println!("The secret number is between 1 and 100.");
            continue;
        }

        println!("You guesses: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}