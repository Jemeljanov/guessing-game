use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Welcome!");

    let secret_number = rand::thread_rng().gen_range(1,100);
    //println!("secret number is {}", secret_number);

    loop {
        println!("Guess the number (from 1 to 100):");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to readline");
        let guess: u32 = match guess.trim().parse(){
            Ok(number) => number,
            Err(_) => continue,
        };
        println!("you guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
                }
        }
    }
}
