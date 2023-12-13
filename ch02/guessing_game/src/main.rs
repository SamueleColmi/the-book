use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    println!("Guess the number");
    println!("The secret number is: {}", secret_number);

    loop {
        let mut guess = String::new();
        println!("\nPlease input your guess");
        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading line");

        let guess: u32 = guess.trim().parse().expect("Input guess is not a number");
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
