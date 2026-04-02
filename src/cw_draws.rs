use crate::{
    config::{
        A_IN_U8, WORDLE_FLIP_DURATION, WORDLE_FLIP_STAGGER, WORDLE_SETTLE_DURATION,
        WORDLE_TRANSLATE_DURATION,
    },
    cw_types::{AttemptAnimation, AttemptAnimationPhase, Progress, WordleInput, WordleLevel},
    helpers::draw_rounded_rect,
};
use macroquad::prelude::*;

pub const GRID_FACTOR: f32 = 11.;
pub const PADDING_PERCENT: f32 = 10.;
const ATTEMPT_SCALE: f32 = 0.6;
const ATTEMPT_TOP_MARGIN: f32 = 24.;
const ATTEMPT_ROW_GAP_FACTOR: f32 = 0.1;
const FACE_LIFT_IN_PADDING: f32 = 2.;
const FACE_LIFT_RATIO: f32 = FACE_LIFT_IN_PADDING * (PADDING_PERCENT / (100. - PADDING_PERCENT));
const SETTLE_DROP_IN_TILE: f32 = 0.3;

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

#[derive(Clone, Copy)]
struct TilePose {
    x: f32,
    face_y: f32,
    tile_size: f32,
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
        Progress::Correct => (DARKGREEN, GREEN, BLACK),
        Progress::Present => (GOLD, YELLOW, BLACK),
        Progress::Absent | Progress::Empty => (GRAY, LIGHTGRAY, BLACK),
    }
}

fn input_tile_pose(col: usize) -> TilePose {
    let Grid {
        tile_size,
        tile_padding,
        ..
    } = get_grid();
    let x_total = 5. * tile_size + 4. * tile_padding;
    let x_rem = screen_width() - x_total;
    let x = col as f32 * (tile_size + tile_padding) + x_rem / 2.;
    let base_y = screen_height() - tile_size - tile_padding;
    let face_y = base_y - tile_size * FACE_LIFT_RATIO;

    TilePose {
        x,
        face_y,
        tile_size,
    }
}

fn attempt_tile_pose(row: usize, col: usize) -> TilePose {
    let Grid {
        full_size,
        tile_size,
        tile_padding,
    } = get_attempt_grid();
    let x_total = 5. * tile_size + 4. * tile_padding;
    let x_start = (screen_width() - x_total) / 2.;
    let row_gap = full_size * ATTEMPT_ROW_GAP_FACTOR;
    let row_height = tile_size + tile_size * FACE_LIFT_RATIO;
    let face_y = ATTEMPT_TOP_MARGIN + row as f32 * (row_height + row_gap);
    let x = x_start + col as f32 * (tile_size + tile_padding);

    TilePose {
        x,
        face_y,
        tile_size,
    }
}

fn push_scaled_rect(
    z_layer: &mut Vec<Vec<Shape>>,
    z: usize,
    rect: Rect,
    color: Color,
    scale_y: f32,
) {
    let scaled_h = rect.h * scale_y;
    let y = rect.y + (rect.h - scaled_h) / 2.;
    z_layer[z].push(Shape::Rectangle(
        Rect {
            x: rect.x,
            y,
            w: rect.w,
            h: scaled_h,
        },
        color,
    ));
}

fn push_tile(
    z_layer: &mut Vec<Vec<Shape>>,
    pose: TilePose,
    colors: (Color, Color, Color),
    letter: char,
    scale_y: f32,
) {
    let lift = pose.tile_size * FACE_LIFT_RATIO;
    let base_rect = Rect {
        x: pose.x,
        y: pose.face_y + lift,
        w: pose.tile_size,
        h: pose.tile_size,
    };
    let face_rect = Rect {
        x: pose.x,
        y: pose.face_y,
        w: pose.tile_size,
        h: pose.tile_size,
    };

    push_scaled_rect(z_layer, 2, base_rect, colors.0, scale_y);
    push_scaled_rect(z_layer, 3, face_rect, colors.1, scale_y);

    if scale_y < 0.2 {
        return;
    }

    let scaled_h = pose.tile_size * scale_y;
    z_layer[4].push(Shape::Letter {
        x: pose.x + pose.tile_size * 0.25,
        y: pose.face_y + (pose.tile_size - scaled_h) / 2. + scaled_h * 0.75,
        font_size: scaled_h,
        letter,
        color: colors.2,
    });
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

fn ease_out_cubic(t: f32) -> f32 {
    1. - (1. - t).powi(3)
}

fn ease_in_out_cubic(t: f32) -> f32 {
    if t < 0.5 {
        4. * t * t * t
    } else {
        1. - (-2. * t + 2.).powi(3) / 2.
    }
}

fn settle_drop(tile_size: f32) -> f32 {
    tile_size * SETTLE_DROP_IN_TILE
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
    for (i, letter_u8) in wordle_input.input.iter().enumerate() {
        if !(*letter_u8 >= A_IN_U8) {
            continue;
        }

        push_tile(
            z_layer,
            input_tile_pose(i),
            (BROWN, BEIGE, DARKBROWN),
            *letter_u8 as char,
            1.,
        );
    }
}

pub fn build_wordle_level(z_layer: &mut Vec<Vec<Shape>>, wordle_level: &WordleLevel) {
    for (row, attempt) in wordle_level.attempts.iter().enumerate() {
        if attempt.word[0] < A_IN_U8 {
            continue;
        }

        for col in 0..5 {
            push_tile(
                z_layer,
                attempt_tile_pose(row, col),
                progress_colors(attempt.progress[col]),
                attempt.word[col] as char,
                1.,
            );
        }
    }
}

pub fn build_wordle_attempt_animation(
    z_layer: &mut Vec<Vec<Shape>>,
    attempt_animation: &AttemptAnimation,
) {
    for col in 0..5 {
        let start_pose = input_tile_pose(col);
        let target_pose = attempt_tile_pose(attempt_animation.target_row, col);
        let hover_face_y = target_pose.face_y + settle_drop(target_pose.tile_size);

        let (pose, colors, scale_y) = match attempt_animation.phase {
            AttemptAnimationPhase::Translate => {
                let t = ease_in_out_cubic(
                    (attempt_animation.phase_elapsed / WORDLE_TRANSLATE_DURATION).clamp(0., 1.),
                );
                (
                    TilePose {
                        x: lerp(start_pose.x, target_pose.x, t),
                        face_y: lerp(start_pose.face_y, hover_face_y, t),
                        tile_size: lerp(start_pose.tile_size, target_pose.tile_size, t),
                    },
                    (BROWN, BEIGE, DARKBROWN),
                    1.,
                )
            }
            AttemptAnimationPhase::Flip => {
                let local_t = ((attempt_animation.phase_elapsed
                    - col as f32 * WORDLE_FLIP_STAGGER)
                    / WORDLE_FLIP_DURATION)
                    .clamp(0., 1.);
                let scale_y = if local_t < 0.5 {
                    1. - local_t * 2.
                } else {
                    (local_t - 0.5) * 2.
                };
                let colors = if local_t < 0.5 {
                    (BROWN, BEIGE, DARKBROWN)
                } else {
                    progress_colors(attempt_animation.progress[col])
                };

                (
                    TilePose {
                        x: target_pose.x,
                        face_y: hover_face_y,
                        tile_size: target_pose.tile_size,
                    },
                    colors,
                    scale_y.max(0.),
                )
            }
            AttemptAnimationPhase::Settle => {
                let t = ease_out_cubic(
                    (attempt_animation.phase_elapsed / WORDLE_SETTLE_DURATION).clamp(0., 1.),
                );
                (
                    TilePose {
                        x: target_pose.x,
                        face_y: lerp(hover_face_y, target_pose.face_y, t),
                        tile_size: target_pose.tile_size,
                    },
                    progress_colors(attempt_animation.progress[col]),
                    1.,
                )
            }
        };

        push_tile(
            z_layer,
            pose,
            colors,
            attempt_animation.guess[col] as char,
            scale_y,
        );
    }
}
