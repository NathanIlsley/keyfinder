use std::io::{self, Write};

fn main() {
    println!("Guess a number!");

    let mut guess = String::new();
    guess = input("Please input your guess: ");

    println!("You guessed: {}", guess);
}

fn input(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().expect("flush failed!");

    let mut inp: String = String::new();

    io::stdin()
        .read_line(&mut inp)
        .expect("Failed to read line!");

    return inp;
}