use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    println!("Guess the number");

    println!("Please input your guess.");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");
    let mut guess = String::new();
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    let guess : u32 = guess
        .trim()
        .parse()
        .expect("Please enter a number");

    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => println!("That's correct!"),
    };

    println!("You guessed: {guess}");
}
