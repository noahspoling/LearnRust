use std::io;
use rand::Rng;

fn main() {
    println!("==========================================================");
    println!("||                                                       ||");
    println!("||                Welcome to Guessing Game               ||");
    println!("||                                                       ||");
    println!("==========================================================");

    // Params
    let mut continue_game = true;
    let mut tries = 3;
    let mut guess_range = 10;
    let mut guess_input = String::new();
    let mut guess_number = 0;
    let mut secret_number = 0;

    // Setup
    secret_number = rand::random::<u32>() % guess_range + 1;


    while continue_game {
        if tries == 0 {
            println!("Out of tries! Generating new secret number...");
            tries = 3;
            secret_number = rand::random::<u32>() % guess_range + 1;
        }
        println!("Guess the number! (1-{})", guess_range);
        println!("You have {} tries left", tries);
        tries -= 1;

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
        if guess_number < secret_number {
            println!("Too small!");
        } else if guess_number > secret_number {
            println!("Too big!");
        } else {
            println!("You win!");
            tries = 3;
            secret_number = rand::random::<u32>() % 10 + 1;
        }
        

        let mut end_game = String::new();
        println!("Do you want to play again? (y/n)");
        io::stdin()
            .read_line(&mut end_game)
            .expect("Failed to read line");

        if end_game.trim() == "n" || end_game.trim() == "N" {
            continue_game = false;
        }
    }
}
