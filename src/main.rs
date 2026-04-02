pub mod config;
pub mod cw_draws;
pub mod cw_types;
pub mod helpers;

use crate::{
    cw_draws::{Shape, draw_wordle_input_bar},
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
    loop {
        clear_background(BLACK);
        let mut z_layers = create_z_layer_vector();
        draw_wordle_input_bar(&mut z_layers);

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
