use crate::tile::Tile;
use macroquad::prelude::*;

pub struct TileInput {
    pub tiles: Vec<Tile>,
    pub cursor: usize,
}

impl TileInput {
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

        Self { tiles, cursor: 0 }
    }

    pub fn handle_letter_add(&mut self) {
        while let Some(key) = get_char_pressed() {
            if key.is_ascii_alphabetic() && self.cursor < 5 {
                let letter = key.to_ascii_uppercase();
                self.tiles[self.cursor].letter = Some(letter);
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
            self.cursor = 0;
            for tile in &mut self.tiles {
                tile.letter = None;
            }
        } else if is_backspace_pressed {
            self.cursor -= 1;
            self.tiles[self.cursor].letter = None;
        }
    }

    pub fn render(&self) {
        for tile in &self.tiles {
            match tile.letter {
                Some(_) => tile.render(),
                None => {}
            }
        }
    }
}
