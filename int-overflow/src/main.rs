use std::io;

fn main() {
    // replicate the overflow
    // what is an overflow?
    // i8 -> 255 -> 256
    let mut a: u8 = 250;

    println!("Please enter a number.");

    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    let user_input: u8 = user_input
        .trim()
        .parse()
        .expect("user_input may not a number");

    a = a + user_input;

    println!("The value of A is {a}");
}
