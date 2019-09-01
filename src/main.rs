use rand::{thread_rng, Rng};
use std::io::{stdout, stdin, Write};

mod guessing_game;
use guessing_game::{GuessResult, GuessingGame};

fn main() {
    let mut game = GuessingGame::new(thread_rng().gen_range(0, 100));
    loop {
        if let Some(guess) = get_guess() {
            let result = game.do_guess(guess);
            match result {
                GuessResult::Correct => {
                    let plural = match game.num_guesses {
                        1 => "guess",
                        _ => "guesses",
                    };
                    println!(
                        "You guessed correctly! Done in {num} {plural}.",
                        num = game.num_guesses,
                        plural = plural
                    );
                    return;
                }
                GuessResult::TooHigh => println!("Too high!"),
                GuessResult::TooLow => println!("Too low!"),
            }
        }
    }
}

fn get_guess() -> Option<i32> {
    print!("Your guess? ");
    stdout().flush().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input)
        .expect("Failed to read line");

    return match input.trim().parse::<i32>() {
        Ok(guess) => Option::Some(guess),
        _ => {
            println!("That was not a number!");
            return Option::None;
        }
    };
}
