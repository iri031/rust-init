use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret = rand::thread_rng()
                        .gen_range(1..=100);

    loop {
        println!("Enter a number:");
        let mut guess = String::new();
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret) {
            Ordering::Less => println!("Less"),
            Ordering::Greater => println!("Greater"),
            Ordering::Equal => {
                println!("You guessed right!");
                break;
            }
        }
    }
    println!("Secret: {secret}");
}
