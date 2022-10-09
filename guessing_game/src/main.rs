use std::io; // so here we're getting the `std` library, wich is the standard library of rust, and from him, we're getting the io, which we use to obtain user input of console

use rand::Rng;
use std::cmp::Ordering;

fn main() {
    // main is entry point in rust
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Please input your guess.");

    let mut guess = String::new(); // mutable variable, with a new instance of String

    io::stdin().read_line(&mut guess).expect("Failed to read"); // using the library io and calling the method stdin that represents a handle to terminal input

    let guess: u32 = guess.trim().parse().expect("Your Guess is not a number");

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Equal => println!("Holly fuck, u good"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Less => println!("Too small!"),
    }
}
