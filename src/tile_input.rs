use crate::{config::WORD_LIST, tile::Tile};
use macroquad::prelude::*;

#[derive(Debug, Default)]
pub struct WordleMode {
    pub input_tiles: Vec<Tile>,
    pub cursor: usize,

    pub curr_level: WordleLevel,
    pub prev_levels: Vec<WordleLevel>,
}

#[derive(Debug, Default)]
pub struct WordleLevel {
    pub solution: [char; 5],
    pub attempts: Vec<[char; 5]>,
    pub progress: Vec<[Progress; 5]>,
}

impl WordleMode {
    pub fn new() -> Self {
        let num_tiles_in_row = 11;
        let border_size = 2.;
        let grid_size = screen_width() / num_tiles_in_row as f32;
        let start_idx = num_tiles_in_row / 2 - 2;
        let end_idx = num_tiles_in_row / 2 + 3;

        let mut tiles = Vec::new();
        for i in start_idx..end_idx {
            let pos = Vec2::new(i as f32, 4.);
            tiles.push(Tile::new_input_tile(
                pos,
                grid_size,
                border_size,
                LIGHTGRAY,
                DARKGRAY,
                WHITE,
                None,
            ));
        }

        Self {
            input_tiles: tiles,
            cursor: 0,
            curr_level: new_wordle_level(0),
            prev_levels: Vec::new(),
        }
    }

    pub fn handle_letter_add(&mut self) {
        while let Some(key) = get_char_pressed() {
            if key.is_ascii_alphabetic() && self.cursor < 5 {
                let letter = key.to_ascii_uppercase();
                self.input_tiles[self.cursor].letter = Some(letter);
                self.cursor += 1;
            }
        }
    }

    pub fn handle_letter_delete(&mut self) {
        let is_cursor_valid = self.cursor > 0;
        if !is_cursor_valid {
            return;
        }

        let is_shift_held = is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift);
        let is_backspace_pressed = is_key_pressed(KeyCode::Backspace);

        if is_shift_held && is_backspace_pressed {
            self.reset_input();
        } else if is_backspace_pressed {
            self.cursor -= 1;
            self.input_tiles[self.cursor].letter = None;
        }
    }

    pub fn handle_wordle_submit(&mut self) {
        if is_key_pressed(KeyCode::Enter) && self.cursor == 5 {
            let attempt = self.get_complete_attempt_from_input();
            let progress = wordle_progress(attempt, self.curr_level.solution);

            self.curr_level.attempts.push(attempt);
            self.curr_level.progress.push(progress);
            self.reset_input();
            println!("{:?}", self);
        }
    }

    fn get_complete_attempt_from_input(&self) -> [char; 5] {
        let mut letters = Vec::new();
        for tile in &self.input_tiles {
            match tile.letter {
                Some(l) => letters.push(l),
                None => {}
            }
        }

        assert!(letters.len() == 5);
        letters.try_into().unwrap()
    }

    fn reset_input(&mut self) {
        self.cursor = 0;
        for tile in &mut self.input_tiles {
            tile.letter = None;
        }
    }

    pub fn render_inputs(&self) {
        for tile in &self.input_tiles {
            match tile.letter {
                Some(_) => tile.render(),
                None => {}
            }
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Progress {
    #[default]
    Empty,
    Absent,
    Present,
    Correct,
}

pub fn new_wordle_level(word_idx: usize) -> WordleLevel {
    let solution: Vec<char> = WORD_LIST[word_idx].to_ascii_uppercase().chars().collect();
    let solution = solution.try_into().unwrap();

    WordleLevel {
        solution,
        attempts: Vec::new(),
        progress: Vec::new(),
    }
}

pub fn wordle_progress(guess: [char; 5], solution: [char; 5]) -> [Progress; 5] {
    let mut result = [Progress::Absent; 5];
    let mut used = [false; 5];

    // First pass: exact matches
    for i in 0..5 {
        if guess[i] == solution[i] {
            result[i] = Progress::Correct;
            used[i] = true;
        }
    }

    // Second pass: present elsewhere
    for i in 0..5 {
        if result[i] == Progress::Correct {
            continue;
        }

        for j in 0..5 {
            if !used[j] && guess[i] == solution[j] {
                result[i] = Progress::Present;
                used[j] = true;
                break;
            }
        }
    }

    result
}
