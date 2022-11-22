use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("\n\n*****The Number Guessing Game*****\n\n");

    let random_num = rand::thread_rng().gen_range(1, 101);

    // println!("The random number is: {} \n", random_num);

    loop {
        let mut guess = String::new();

        println!("Guess a number: ");

        // User Input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to get input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("\nYou have entered: {}\n", guess);

        match guess.cmp(&random_num) {
            Ordering::Less => println!("Too Small"),
            Ordering::Equal => {
                println!("Woo hoo! You guessed it right");
                break;
            }
            Ordering::Greater => println!("Too Large"),
        };

        println!();
    }
}
