use crate::cw_types::WordleInput;
use macroquad::prelude::*;

pub const GRID_FACTOR: f32 = 11.;
pub const PADDING_PERCENT: f32 = 5.;

pub enum Shape {
    Rectangle(Rect, Color),
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
    let base = Shape::Rectangle(Rect { x, y, w, h }, WHITE);
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

pub fn draw_wordle_input_tiles(z_layer: &mut Vec<Vec<Shape>>, wordle_input: &WordleInput) {}
