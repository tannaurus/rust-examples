use std::io;

fn main() {
    // An integer overflow occurs when you increment an integer past the bounds
    // that the declared type expects.
    // https://doc.rust-lang.org/book/ch03-02-data-types.html

    // Here are some examples of those bounds
    // i8: (-127 .. 126)
    // u8: (0 .. 255)
    // i16: (−32,768 .. −32,767)
    // u16: (0 .. 65,536)

    // Since we're using u8:
    // adding 5 to this will be valid
    // adding 6 to this will cause an integer overflow
    let mut observed_integer: u8 = 250;

    println!("Enter a number greater than 5 to observe the integer overflow behavior");

    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    let user_input: u8 = user_input
        .trim()
        .parse()
        .expect("user_input may not a number");

    // a debug build will panic here if you enter a value greater than 5
    observed_integer = observed_integer + user_input;

    // a release build will wrap the user's input
    println!("The value of our observed integer is now {observed_integer}.");
    println!("Goodbye.")
}
