use crate::{
    config::A_IN_U8,
    cw_types::{Progress, WordleInput, WordleLevel},
    helpers::draw_rounded_rect,
};
use macroquad::prelude::*;

pub const GRID_FACTOR: f32 = 11.;
pub const PADDING_PERCENT: f32 = 10.;
const ATTEMPT_SCALE: f32 = 0.6;
const ATTEMPT_TOP_MARGIN: f32 = 24.;
const ATTEMPT_ROW_GAP_FACTOR: f32 = 0.1;
const FACE_LIFT_IN_PADDING: f32 = 2.;

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

pub fn draw_all_shapes(z_layers: &Vec<Vec<Shape>>) {
    for z in z_layers {
        for shape in z {
            match *shape {
                Shape::Rectangle(Rect { x, y, w, h }, color) => {
                    draw_rounded_rect(x, y, w, h, 5., color);
                }
                Shape::Letter {
                    x,
                    y,
                    font_size,
                    letter,
                    color,
                } => {
                    draw_text(letter.to_string().as_str(), x, y, font_size, color);
                }
            }
        }
    }
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

fn get_attempt_grid() -> Grid {
    let grid = get_grid();
    let full_size = grid.full_size * ATTEMPT_SCALE;
    let tile_padding = full_size * (PADDING_PERCENT / 100.);
    let tile_size = full_size - tile_padding;

    Grid {
        full_size,
        tile_size,
        tile_padding,
    }
}

fn progress_colors(progress: Progress) -> (Color, Color, Color) {
    match progress {
        Progress::Correct => (DARKGREEN, GREEN, DARKBROWN),
        Progress::Present => (GOLD, YELLOW, DARKBROWN),
        Progress::Absent | Progress::Empty => (DARKGRAY, GRAY, WHITE),
    }
}

pub fn build_wordle_input_bar(z_layer: &mut Vec<Vec<Shape>>) {
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

pub fn build_wordle_input_tiles(z_layer: &mut Vec<Vec<Shape>>, wordle_input: &WordleInput) {
    let Grid {
        tile_size,
        tile_padding,
        ..
    } = get_grid();

    let x_total = 5. * tile_size + 4. * tile_padding;
    let x_rem = screen_width() - x_total;

    for (i, letter_u8) in wordle_input.input.iter().enumerate() {
        if !(*letter_u8 >= A_IN_U8) {
            continue;
        }

        let x = i as f32 * (tile_size + tile_padding) + x_rem / 2.;
        let base_y = screen_height() - tile_size - tile_padding;
        let w = tile_size;
        let h = tile_size;
        let base = Shape::Rectangle(Rect { x, y: base_y, w, h }, BROWN);
        z_layer[2].push(base);

        let face_y = base_y - tile_padding * FACE_LIFT_IN_PADDING;
        let face = Shape::Rectangle(Rect { x, y: face_y, w, h }, BEIGE);
        z_layer[3].push(face);

        let x = x + tile_size * 0.25;
        let y = face_y + tile_size * 0.75;
        let letter = Shape::Letter {
            x,
            y,
            font_size: tile_size,
            letter: *letter_u8 as char,
            color: DARKBROWN,
        };
        z_layer[4].push(letter);
    }
}

pub fn build_wordle_level(z_layer: &mut Vec<Vec<Shape>>, wordle_level: &WordleLevel) {
    let Grid {
        full_size,
        tile_size,
        tile_padding,
    } = get_attempt_grid();

    let x_total = 5. * tile_size + 4. * tile_padding;
    let x_start = (screen_width() - x_total) / 2.;
    let row_gap = full_size * ATTEMPT_ROW_GAP_FACTOR;
    let row_height = tile_size + tile_padding * FACE_LIFT_IN_PADDING;

    for (row, attempt) in wordle_level.attempts.iter().enumerate() {
        if attempt.word[0] < A_IN_U8 {
            continue;
        }

        let face_y = ATTEMPT_TOP_MARGIN + row as f32 * (row_height + row_gap);
        let base_y = face_y + tile_padding * FACE_LIFT_IN_PADDING;

        for col in 0..5 {
            let x = x_start + col as f32 * (tile_size + tile_padding);
            let w = tile_size;
            let h = tile_size;
            let (base_color, face_color, letter_color) = progress_colors(attempt.progress[col]);

            z_layer[2].push(Shape::Rectangle(Rect { x, y: base_y, w, h }, base_color));
            z_layer[3].push(Shape::Rectangle(Rect { x, y: face_y, w, h }, face_color));
            z_layer[4].push(Shape::Letter {
                x: x + tile_size * 0.25,
                y: face_y + tile_size * 0.75,
                font_size: tile_size,
                letter: attempt.word[col] as char,
                color: letter_color,
            });
        }
    }
}
