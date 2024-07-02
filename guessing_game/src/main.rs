use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Let's play a game!");
    let secret = thread_rng().gen_range(1..=100);
    println!("I have chosen a number between 1 and 100. You guess it.");
    loop {
        println!("Input a guess");
        let mut guess = String::new();
        let read_result = io::stdin().read_line(&mut guess);
        if read_result.is_err() {
            println!("Unfortunately, I failed to read your input. :( Try again?");
            continue;
        }
        let Ok(guess) = guess.trim().parse::<u32>() else {
            println!("Unfortunately, I could not parse your input as a number. :( Try again?");
            continue;
        };
        match guess.cmp(&secret) {
            Ordering::Less => println!("You guessed too low. Try higher."),
            Ordering::Equal => {
                println!("You found the number! It was {secret}. Congratulations! :D");
                break;
            }
            Ordering::Greater => println!("You guessed too high. Try lower."),
        }
    }
}
