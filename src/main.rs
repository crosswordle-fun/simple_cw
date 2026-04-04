pub mod config;
pub mod cw_draws;
pub mod cw_inputs;
pub mod cw_types;
pub mod helpers;
pub mod tile;

use crate::tile::Tile;
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

    let num_tiles_in_row = 11;
    let grid_size = screen_width() / num_tiles_in_row as f32;
    let start_idx = num_tiles_in_row / 2 - 2;
    let end_idx = num_tiles_in_row / 2 + 3;

    let mut tiles = Vec::new();
    for i in start_idx..end_idx {
        let pos = Vec2::new(i as f32, 4.);
        tiles.push(Tile::new_input_tile(
            pos, grid_size, LIGHTGRAY, DARKGRAY, WHITE,
        ));
    }

    loop {
        clear_background(BLACK);

        for t in &tiles {
            t.render();
        }
        draw_fps();
        next_frame().await;
    }
}
