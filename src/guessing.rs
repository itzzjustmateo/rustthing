use std::cmp::Ordering;
use std::io::{self, Write};
use std::{thread, time};

fn main() {
    println!("Guess the number!");

    let secret_number = rand::random_range(1..=100);

    loop {
        print!("Please input your guess: ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                thread::sleep(time::Duration::from_millis(450));
                println!("Please input a number");
                thread::sleep(time::Duration::from_millis(450));
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You won!");
                break;
            }
        }
    }
}
