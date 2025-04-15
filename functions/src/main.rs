fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    let mut temp = x;
    temp + 1;
    temp + 1
}

fn plus_one(x: i32) -> i32 {
    let temp = x + 1;
    temp + 1
}
