use std::io;

fn main() {
    println!("How many spaces do you want?");

    let mut spaces = String::new();

    io::stdin()
        .read_line(&mut spaces)
        .expect("Failed to read line");
    let spaces = spaces.len();

    println!("You typed {} spaces", spaces);
    let number ="5";

    let guess: u32 = number.parse().expect("Not a number!");
    println!("Your guess is {}", guess);
}
