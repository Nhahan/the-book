use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101); // immutable variable
    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new(); // mutable variable

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess = guess.trim().parse().expect("Please type a number!"); // shadowing

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"), // match arms
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
