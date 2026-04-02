use crate::{config::A_IN_U8, cw_types::WordleInput};
use macroquad::prelude::*;

pub const GRID_FACTOR: f32 = 11.;
pub const PADDING_PERCENT: f32 = 10.;

pub enum Shape {
    Rectangle(Rect, Color),
    Letter {
        x: f32,
        y: f32,
        font_size: f32,
        letter: char,
        color: Color,
    },
}

pub struct Grid {
    full_size: f32,
    tile_size: f32,
    tile_padding: f32,
}

fn get_grid() -> Grid {
    let full_size = screen_width() / GRID_FACTOR;
    let tile_padding = full_size * (PADDING_PERCENT / 100.);
    let tile_size = full_size - tile_padding;

    Grid {
        full_size,
        tile_size,
        tile_padding,
    }
}

pub fn draw_wordle_input_bar(z_layer: &mut Vec<Vec<Shape>>) {
    let Grid {
        full_size,
        tile_size,
        tile_padding,
    } = get_grid();

    let x_total = 5. * tile_size + 4. * tile_padding;
    let x_rem = screen_width() - x_total;

    let x = x_rem / 2. - tile_padding;
    let y = screen_height() - full_size - tile_padding;
    let w = x_total + tile_padding * 2.;
    let h = full_size + tile_padding;
    let base = Shape::Rectangle(Rect { x, y, w, h }, DARKBROWN);
    z_layer[0].push(base);

    for i in 0..5 {
        let x = i as f32 * (tile_size + tile_padding) + x_rem / 2.;
        let y = screen_height() - tile_padding - tile_size;
        let w = tile_size;
        let h = tile_size;
        let tile = Shape::Rectangle(Rect { x, y, w, h }, BLACK);

        z_layer[1].push(tile);
    }
}

pub fn draw_wordle_input_tiles(z_layer: &mut Vec<Vec<Shape>>, wordle_input: &WordleInput) {
    let Grid {
        full_size,
        tile_size,
        tile_padding,
    } = get_grid();

    let x_total = 5. * tile_size + 4. * tile_padding;
    let x_rem = screen_width() - x_total;

    for (i, letter_u8) in wordle_input.input.iter().enumerate() {
        if !(*letter_u8 >= A_IN_U8) {
            continue;
        }

        let x = i as f32 * (tile_size + tile_padding) + x_rem / 2.;
        let y = screen_height() - tile_size - tile_padding;
        let w = tile_size;
        let h = tile_size;
        let base = Shape::Rectangle(Rect { x, y, w, h }, BROWN);
        z_layer[2].push(base);

        let y = screen_height() - tile_size - tile_padding * 3.;
        let face = Shape::Rectangle(Rect { x, y, w, h }, BEIGE);
        z_layer[2].push(face);

        let x = x + tile_size * 0.25;
        let y = y + tile_size * 0.75;
        let letter = Shape::Letter {
            x,
            y,
            font_size: tile_size,
            letter: *letter_u8 as char,
            color: DARKBROWN,
        };
        z_layer[3].push(letter);
    }
}
