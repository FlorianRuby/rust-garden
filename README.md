### Functions

```rust
fn main() { 
println!("Hello World");
}
```

**fn** to define functions. 

**name** after fn (if name = **main** ⇒ gets called as the first function in the entire project)

**()** = parameters, return values 

### Console Print

```rust
println!("Print with linewrap");
print!("without linewrap");
```

### Macro !

```rust
println!("This calls a macro, indicated by the !"); 
println("This calls a function, indicated by the nonexistent !"); 
```

### Cargo

Cargo is used for projects with dependencies (so most of the projects except really simple ones)

initialize a cargo project with 

```bash
cargo new tunnel_name
```

The project now looks like this: 

in the /src folder all the rust scripts are placed, in the cargo.toml file all the dependencides are placed and general informations like rust version. 

![image](https://github.com/user-attachments/assets/d1ad80d7-2e61-4f14-ad31-f418f5d9c373)

The content looks like the following

### Add dependencies

```toml
[dependencies]
rand = "0.8.5" # add at the bottom
```

### Convert non cargo to cargo project

```bash
cargo init
```

Simply move all the .rs files into the /src folder and create the cargo.toml file

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2024"

[dependencies]
```

### How to run a simple .rs file

```bash
rustc main.rs # creates compiled .exe file of the code, can me run without rust installed
.\main.exe    # executes said .exe file
```

### How to run a cargo project

```bash
cargo run 

# long form:
cargo build
.\target\debug\hello_cargo.exe # takes project name
```

### Check if the code is compilable

```bash
cargo check
```

Pros compared to cargo run: much faster since it doesn’t produce an executable.

### Build cargo project for release

```bash
cargo build --release
```

Why? Applies more optimisations thus runs better, however it takes longer to compile. 

### Variables

```rust
let x = 5; // immutable = can't change value later
let mut y = 5; // mutable = can change value later

let mut var_name = String::new();
```

In Rust variables are generally not changeable after they’ve been set = immutable.

If we add the mut after the let we make the variable mutable, meaning we can change the value later on.

With :: we call an associated function ⇒ ::new, this creates a new empty string.

### Read console line as input and &

```rust
io::stdin()
    .read_line(&mut guess)
```

With read_line we can store the input of the user in the following parameter ().

The & indicates that the following argument is a reference which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times.

<aside>
&var_name are by default immutable, use &mut var_name to make them mutable
</aside>

### Catching errors

```rust
io::stdin()
    .read_line(&mut guess)
		.expect("There have been some issues");
```

In Rust there are enums, basically error handling, the value can either be OK or ERR. We get an enum from the .read_line snippet before. (Enums get created along side results). The .expect catches and stops the programming if the enum value is ERR. 

### Returning values using println!

```rust
    println!("You've guessed: {}", guess);
```

The {} needs to be placed where the value should be shown. After the “” we use a comma and then the value that the {} should use. 

With multiple {} it would look like this, 

```rust
    println!("{} you've guessed: {}", name, guess); // use the same order
    
    // You can also just put the name in the {}
    
    println!("{name} you've guessed: {guess}");
```

### Random numbers

```rust
use Rand::Rng;

let secret_number = rand::thread_rng().gen_range(1..=100);
```

### Enum

Enum = possible variations

Result has ERR, OK for example, Ordering has Greater, Less & Equal f.e.

## Variables

### Scalar

A *scalar* type represents a single value. Rust has four primary scalar types: 

1. integers
2. floating-point numbers 
3. Booleans 
4. characters. 

### Integer

An *integer* is a number without a fractional component. We used one integer type in Chapter 2, the `u32` type. This type declaration indicates that the value it’s associated with should be an unsigned integer (signed integer types start with `i` instead of `u`) that takes up 32 bits of space. 

| 8-bit | `i8` | `u8` |
| --- | --- | --- |
| 16-bit | `i16` | `u16` |
| 32-bit | `i32` | `u32` |
| 64-bit | `i64` | `u64` |
| 128-bit | `i128` | `u128` |
| arch | `isize` | `usize` |

Control flow is the mechanism that allows you to run code based on conditions or repeatedly while a condition is true. Rust provides key constructs such as `if` expressions and loops to manage control flow.

## if Expressions

An `if` expression helps you execute code based on a condition. If the condition is true, the block of code inside the `if` runs; otherwise, it doesn't.

```rust
fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
```

- The condition must be a `bool`.
- If the condition evaluates to false and no `else` block is provided, the program skips the `if` block.

### Multiple Conditions with `else if`

You can chain conditions using `else if`:

```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

Rust executes the first `true` condition and skips the rest.

### Using `if` in a `let` Statement

Since `if` is an expression, you can use it in a `let` statement:

```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}
```

Ensure both arms of the `if` have the same return type, or you’ll get a compile-time error.

## Loops

Rust provides three types of loops: `loop`, `while`, and `for`.

### `loop` (Infinite Loop)

A `loop` runs indefinitely until explicitly broken.

```rust
fn main() {
    loop {
        println!("again!");
    }
}
```

To exit the loop, use `break`, or to skip an iteration, use `continue`.

### Returning Values from Loops

You can return a value from a loop by using the `break` keyword with a value:

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
```

### Loop Labels

When you have nested loops, you can label them to control which loop `break` or `continue` applies to:

```rust
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
```

### `while` Loop (Conditional Loop)

A `while` loop runs while a condition is true.

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    println!("LIFTOFF!!!");
}
```

### `for` Loop (Iterating over Collections)

The `for` loop is the safest and most concise way to iterate over collections, like arrays.

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

```

### Range with `for`

You can use `for` with ranges to loop through numbers:

```rust
fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
```
