pub mod config;
pub mod cw_draws;
pub mod cw_types;
pub mod helpers;

use crate::{
    cw_draws::{build_wordle_input_bar, build_wordle_input_tiles, draw_all_shapes},
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
    // set_default_filter_mode(FilterMode::Nearest);

    let wordle_input = WordleInput {
        input: [65, 66, 67, 68, 69],
        cursor: 0,
    };

    loop {
        clear_background(BLACK);
        let mut z_layers = create_z_layer_vector();

        build_wordle_input_bar(&mut z_layers);
        build_wordle_input_tiles(&mut z_layers, &wordle_input);

        draw_all_shapes(&z_layers);

        draw_fps();
        next_frame().await;
    }
}
