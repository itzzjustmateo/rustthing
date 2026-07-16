use std::{
    io::{self, Write},
    thread, time,
};

fn main() {
    let number: i32;

    loop {
        print!("Please input a number: ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse() {
            Ok(num) => {
                number = num;
                break;
            }
            Err(_) => {
                thread::sleep(time::Duration::from_millis(450));
                println!("That's not a valid number! Try again.");
                thread::sleep(time::Duration::from_millis(450));
                continue;
            }
        };
    }

    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 4, 3, or 2");
    }
}
