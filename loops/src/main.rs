use std::io;

fn main() {
    helper();
}
fn helper() {
    let mut celsius= String::new();
    println!("Enter Celsius amount: ");
        io::stdin()
            .read_line(&mut celsius)
            .expect("Failed to read line");
    let celsius: i32 = celsius.trim().parse().expect("Please type a number!");
    let fahrenheit = (celsius * 9/5) + 32;

    println!("{} Celsius is {} in Fahrenheit", celsius, fahrenheit);
}