use crate::{config::WORD_LIST, tile::Tile};
use macroquad::prelude::*;

#[derive(Debug, Default)]
pub struct WordleMode {
    pub input_tiles: Vec<Tile>,
    pub cursor: usize,

    pub curr_level: WordleLevel,
    pub prev_levels: Vec<WordleLevel>,
    pub attempt_tiles: Vec<Tile>,
}

#[derive(Debug, Default)]
pub struct WordleLevel {
    pub solution: [char; 5],
    pub attempts: Vec<[char; 5]>,
    pub progress: Vec<[Progress; 5]>,
}

impl WordleMode {
    pub fn new() -> Self {
        let input_tiles = Self::build_input_tiles();
        let curr_level = new_wordle_level(0);

        Self {
            input_tiles,
            cursor: 0,
            curr_level,
            prev_levels: Vec::new(),
            attempt_tiles: Vec::new(),
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

            let mut attempt_tiles = self.build_attempt_tiles(&attempt, &progress);
            self.attempt_tiles.append(&mut attempt_tiles);
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

    pub fn render_attempts(&self) {
        for tile in &self.attempt_tiles {
            tile.render();
        }
    }

    fn build_input_tiles() -> Vec<Tile> {
        let num_tiles_in_row = 11;
        let border_size = 2.;
        let grid_size = screen_width() / num_tiles_in_row as f32;
        let start_idx = num_tiles_in_row / 2 - 2;
        let end_idx = num_tiles_in_row / 2 + 3;
        let border_c = BLACK;

        let mut tiles = Vec::new();
        for i in start_idx..end_idx {
            let pos = Vec2::new(i as f32, 4.);
            let (face_c, base_c) = Self::get_tile_colors_from_progress(&Progress::Empty);
            tiles.push(Tile::new_input_tile(
                pos,
                grid_size,
                border_size,
                face_c,
                base_c,
                border_c,
                None,
            ));
        }

        tiles
    }

    fn build_attempt_tiles(&self, attempt: &[char; 5], progress: &[Progress; 5]) -> Vec<Tile> {
        let mut attempt_tiles = Vec::new();
        let attempt_num = self.curr_level.attempts.len() + 1;
        let num_tiles_in_row = 15;
        let start_idx = num_tiles_in_row / 2 - 2;
        let border_size = 2.;
        let size = screen_width() / num_tiles_in_row as f32;
        let border_c = BLACK;

        for (i, (l, p)) in attempt.iter().zip(progress.iter()).enumerate() {
            let pos = Vec2 {
                x: (i + start_idx) as f32,
                y: attempt_num as f32,
            };

            let (face_c, base_c) = Self::get_tile_colors_from_progress(p);
            let tile =
                Tile::new_input_tile(pos, size, border_size, face_c, base_c, border_c, Some(*l));

            attempt_tiles.push(tile);
        }

        attempt_tiles
    }

    fn get_tile_colors_from_progress(progress: &Progress) -> (Color, Color) {
        match progress {
            Progress::Empty => (WHITE, LIGHTGRAY),
            Progress::Absent => (LIGHTGRAY, GRAY),
            Progress::Present => (YELLOW, GOLD),
            Progress::Correct => (GREEN, DARKGREEN),
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
