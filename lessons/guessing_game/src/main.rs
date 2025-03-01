use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("==========================================================");
    println!("||                                                       ||");
    println!("||                Welcome to Guessing Game               ||");
    println!("||                                                       ||");
    println!("==========================================================");

    #[cfg(target_pointer_width = "64")]
    type UInt = u64;

    #[cfg(target_pointer_width = "32")]
    type UInt = u32;

    // Params
    let mut continue_game = true;
    let guess_range : UInt = 10;
    
    
    while continue_game {

        let mut tries : UInt = 3;
        let mut guess_input = String::new();
        let mut guess_number : UInt = 0;
        let mut secret_number : UInt = 0;

        // Setup
        secret_number = rand::random::<UInt>() % guess_range + 1;

        while tries > 0 {
            println!("Guess the number! (1-{})", guess_range);
            println!("You have {} tries left", tries);

            guess_input.clear();

            io::stdin()
                .read_line(&mut guess_input)
                .expect("Failed to read line");

            guess_number = match guess_input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a valid number!");
                    continue;
                }
            };

            println!("You guessed: {}", guess_input);
            
            match guess_number.cmp(&secret_number) {
                Ordering::Less => { println!("Too small!"); }
                Ordering::Greater => { println!("Too big!"); }
                Ordering::Equal => {
                    println!("You win!");
                    break;
                }
            };
            tries -= 1;
        }

        if tries == 0 {
            println!("Out of tries! The secret number was: {}", secret_number);
        }

        let mut end_game = String::new();
        loop {
            println!("Do you want to play again? (y/n)");
            io::stdin()
                .read_line(&mut end_game)
                .expect("Failed to read line");

            match end_game.trim().to_lowercase().as_str() {
                "y" => {
                    continue_game = true;
                    break;
                }
                "n" => {
                    continue_game = false;
                    break;
                }
                _ => {
                    println!("Please enter 'y' or 'n'.");
                }
            }
        }
    }
}
