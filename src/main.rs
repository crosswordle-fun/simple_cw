pub mod config;
pub mod cw_draws;
pub mod cw_inputs;
pub mod cw_types;
pub mod helpers;
pub mod tile;
pub mod tile_input;

use crate::{tile::Tile, tile_input::TileInput};
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "game".to_owned(),
        window_width: 1280,
        window_height: 720,
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

    let mut new_wordle_input = TileInput::new();
    loop {
        clear_background(BLACK);

        new_wordle_input.handle_letter_add();
        new_wordle_input.handle_letter_delete();
        for tile in &new_wordle_input.tiles {
            match tile.letter {
                Some(_) => tile.render(),
                None => {}
            }
        }

        draw_fps();
        next_frame().await;
    }
}
