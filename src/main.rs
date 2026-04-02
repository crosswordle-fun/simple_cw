pub mod config;
pub mod cw_draws;
pub mod cw_types;
pub mod helpers;

use crate::{
    config::A_IN_U8,
    cw_draws::{Shape, draw_wordle_input_bar, draw_wordle_input_tiles},
    cw_types::WordleInput,
    helpers::{create_z_layer_vector, draw_rounded_rect},
};
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "game".to_owned(),
        platform: miniquad::conf::Platform {
            linux_backend: miniquad::conf::LinuxBackend::WaylandOnly,
            ..Default::default()
        },
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // set_default_filter_mode(FilterMode::Nearest);

    let mut wordle_input = WordleInput {
        input: [65, 66, 67, 68, 69],
        cursor: 0,
    };

    loop {
        clear_background(BLACK);
        let mut z_layers = create_z_layer_vector();

        draw_wordle_input_bar(&mut z_layers);
        draw_wordle_input_tiles(&mut z_layers, &wordle_input);

        for z in &z_layers {
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

        draw_fps();
        next_frame().await;
    }
}
