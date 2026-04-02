pub mod config;
pub mod cw_draws;
pub mod cw_types;
pub mod helpers;

use crate::{
    cw_draws::{Shape, draw_wordle_input_bar, draw_wordle_input_tiles},
    cw_types::WordleInput,
    helpers::create_z_layer_vector,
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
    let mut wordle_input = WordleInput {
        input: [0; 5],
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
                    Shape::Rectangle(rect, color) => {
                        draw_rectangle(rect.x, rect.y, rect.w, rect.h, color);
                    }
                }
            }
        }

        draw_fps();
        next_frame().await;
    }
}
