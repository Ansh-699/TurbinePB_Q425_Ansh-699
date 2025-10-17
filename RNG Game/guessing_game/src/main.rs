use std::io;
use std::cmp::Ordering;

fn main() {
    println!("-----------------Guessing Game-------------------");
    let secret_number: u32 = rand::random_range(0..=100); 
    println!("--Random number generated, take a guess!---");

    let mut counter = 0;

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Error while reading input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        counter += 1; 

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You got it in {} tries!", counter);
                println!("----------Game Ended------------");
                break;
            }
            Ordering::Greater => {
                println!("Too high!");
            }
            Ordering::Less => {
                println!("Too low!");
            }
        }
    }
}