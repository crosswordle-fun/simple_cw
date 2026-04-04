pub mod config;
pub mod cw_draws;
pub mod cw_inputs;
pub mod cw_types;
pub mod helpers;
pub mod tile;

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

    loop {
        clear_background(BLACK);
        draw_fps();
        next_frame().await;
    }
}
