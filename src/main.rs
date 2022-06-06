use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number (from 1 to 10):");

    let secret_number = rand::thread_rng().gen_range(1,10);
    println!("secret number is {}", secret_number);

    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Failed to readline");
    println!("you guessed {}", guess);

}
