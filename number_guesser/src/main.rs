use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Guess the number!");

    loop {
        println!("Please input your guess.");
        
        let mut guess = String::new();
        io::stdin() // We're now using an 'import' so we're stating where we have the following dependencies from using io::stdin
            .read_line(&mut guess)
            .expect("Failed to read line");
        println!("You've guessed: {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => { 
                println!("You've guessed the correct number!");
                break;
            }
        }
    }
}
