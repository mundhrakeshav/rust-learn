use rand::Rng;
use std::cmp::Ordering;
use std::io;
// 0x000000016fdf6490
fn main() {
    let mut guess: String = String::new();
    loop {
        println!("Guess the number!");

        println!("Please input your guess.");
        let secret_number = rand::thread_rng().gen_range(1..=100);

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid Number");
                continue;
            }
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        println!("You guessed: {secret_number}, {guess}");
    }
}
