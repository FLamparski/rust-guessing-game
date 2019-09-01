use std::cmp::Ordering;

#[derive(Debug)]
pub struct GuessingGame {
    target: i32,
    pub num_guesses: i32,
}

pub enum GuessResult {
    TooLow,
    TooHigh,
    Correct,
}

impl GuessingGame {
    pub fn new(target: i32) -> GuessingGame {
        GuessingGame {
            target: target,
            num_guesses: 0,
        }
    }

    pub fn do_guess(&mut self, guess: i32) -> GuessResult {
        self.num_guesses += 1;
        return match guess.cmp(&self.target) {
            Ordering::Equal => GuessResult::Correct,
            Ordering::Greater => GuessResult::TooHigh,
            Ordering::Less => GuessResult::TooLow
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increments_num_guesses() {
        let mut game = GuessingGame::new(10);
        game.do_guess(5);
        assert_eq!(game.num_guesses, 1);
    }

    #[test]
    fn correct_result() {
        let mut game = GuessingGame::new(10);
        let result = game.do_guess(10);
        match result {
            GuessResult::Correct => assert!(true),
            _ => assert!(false),
        };
    }

    #[test]
    fn too_high_result() {
        let mut game = GuessingGame::new(10);
        let result = game.do_guess(11);
        match result {
            GuessResult::TooHigh => assert!(true),
            _ => assert!(false),
        };
    }

    #[test]
    fn too_low_result() {
        let mut game = GuessingGame::new(10);
        let result = game.do_guess(9);
        match result {
            GuessResult::TooLow => assert!(true),
            _ => assert!(false),
        };
    }
}