use rand::Rng;
use std::cmp::Ordering;
use std::io;
use colored::*;

fn main() {
    println!("Welcome to Guessing Game!");

    let random_num = rand::thread_rng().gen_range(1..=100);
    println!("The random number is: {random_num}");

    loop {
        println!("Enter the input: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter a valid number!\n");
                continue;
            },
        }; 

        println!("You guessed: {guess}");

        match guess.cmp(&random_num) {
            Ordering::Less => println!("{}", "Too low :(".red()),
            Ordering::Greater => println!("{}", "Too high :(".yellow()),
            Ordering::Equal => {
                println!("{}", "Correct!!".green());
                break;
            }
        }
    }
}
