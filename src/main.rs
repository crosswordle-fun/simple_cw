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
        next_frame().await;
    }
}
