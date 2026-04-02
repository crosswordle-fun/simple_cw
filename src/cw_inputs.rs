use crate::cw_types::WordleInput;
use macroquad::prelude::*;

pub fn handle_wordle_input(wordle_input: &mut WordleInput) {
    if is_key_pressed(KeyCode::Backspace) {
        println!("pressed backspace");
        if wordle_input.cursor > 0 {
            wordle_input.cursor -= 1;
            wordle_input.input[wordle_input.cursor as usize] = 0;
        }
    }

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
}
