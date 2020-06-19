use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to the Guessing Game!!!!");

    let secret_number = rand::thread_rng().gen_range(1, 100);
    loop {
        println!("Please Enter a number: ");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!!"),
            Ordering::Greater => println!("Too Large!!"),
            Ordering::Equal => {
                println!("You Win!!");
                break;
            }
        }
    }
}
