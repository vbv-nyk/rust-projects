use rand::{self, Rng};
use std::io;

fn main() {
    let mut rng = rand::thread_rng();
    let secret_number = rng.gen_range(1..10);
    // print!("The random floating number generated is {}", secret_number);

    loop {
        let mut guess = String::new();
        println!("Enter your guess: ");
        io::stdin()
            .read_line(&mut guess)
            .expect("Please enter a valid number");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                print!("There was an error parsing your number, try again.");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Equal => {
                println!("You won!!");
                break;
            }
            std::cmp::Ordering::Greater => println!("Your guess was higher"),
            std::cmp::Ordering::Less => println!("Your guess was lower"),
        }
    }

    println!("Congratulations, you've won the game");
}
