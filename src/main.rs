pub mod config;
pub mod cw_draws;
pub mod cw_inputs;
pub mod cw_types;
pub mod helpers;

use crate::{
    cw_draws::{
        build_wordle_attempt_animation, build_wordle_input_bar, build_wordle_input_tiles,
        build_wordle_level, draw_all_shapes,
    },
    cw_inputs::{handle_wordle_input, update_wordle_animation},
    cw_types::{WordleGame, WordleInput, new_wordle_level},
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
    set_default_filter_mode(FilterMode::Nearest);

    let mut wordle_input = WordleInput::default();
    let mut wordle_game = WordleGame::default();
    wordle_game.curr_level = new_wordle_level(0);
    println!("{:#?}", wordle_game);

    loop {
        handle_wordle_input(&mut wordle_input, &mut wordle_game);
        update_wordle_animation(&mut wordle_game);

        let mut z_layers = create_z_layer_vector();
        clear_background(BLACK);
        build_wordle_level(&mut z_layers, &wordle_game.curr_level);
        build_wordle_input_bar(&mut z_layers);
        build_wordle_input_tiles(&mut z_layers, &wordle_input);
        if let Some(attempt_animation) = wordle_game.active_animation.as_ref() {
            build_wordle_attempt_animation(&mut z_layers, attempt_animation);
        }
        draw_all_shapes(&z_layers);
        draw_fps();

        next_frame().await;
    }
}
