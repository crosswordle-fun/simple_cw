use crate::{
    cw_types::{Attempt, WordleInput, WordleLevel},
    helpers::wordle_progress,
};
use macroquad::prelude::*;

pub fn handle_wordle_input(wordle_input: &mut WordleInput, wordle_level: &mut WordleLevel) {
    while let Some(c) = get_char_pressed() {
        if c.is_ascii_alphabetic() {
            println!("pressed {c}");
            let letter = c.to_ascii_uppercase();
            if wordle_input.cursor < 5 {
                wordle_input.input[wordle_input.cursor as usize] = letter as u8;
                wordle_input.cursor += 1;
            }
        }
    }

    if is_key_pressed(KeyCode::Backspace) {
        println!("pressed backspace");
        if wordle_input.cursor > 0 {
            wordle_input.cursor -= 1;
            wordle_input.input[wordle_input.cursor as usize] = 0;
        }
    }

    if is_key_pressed(KeyCode::Enter) {
        println!("pressed enter");
        if wordle_input.cursor == 5 {
            for i in 0..wordle_level.attempts.len() {
                if wordle_level.attempts[i].word[0] != 0 {
                    continue;
                }

                let solution = wordle_level.solution;
                let guess = wordle_input.input;

                let mut new_attempt = Attempt::default();
                new_attempt.word = guess;
                new_attempt.progress = wordle_progress(guess, solution);
                wordle_level.attempts[i] = new_attempt;
                break;
            }
        }
        println!("{:#?}", wordle_level);
    }
}
