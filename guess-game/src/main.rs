use std::{cmp::Ordering, io};

use rand::{thread_rng, Rng};

fn main() {
    println!("Welcome to Guessing game !!");

    let secret_num = thread_rng().gen_range(1..=100);

    loop {
        let mut guess_str = String::new();
        println!("Enter a number");
        io::stdin()
            .read_line(&mut guess_str)
            .expect("Failed to read line");

        let guess: u32 = match guess_str.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You guessed it right !!");
                break;
            }
        };
    }
}
