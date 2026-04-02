use crate::{config::Z_LAYERS, cw_draws::Shape};
use macroquad::prelude::*;

pub fn create_z_layer_vector() -> Vec<Vec<Shape>> {
    let mut z_layer: Vec<Vec<Shape>> = Vec::with_capacity(Z_LAYERS);
    for _ in 0..Z_LAYERS {
        z_layer.push(Vec::with_capacity(Z_LAYERS));
    }

    z_layer
}

pub fn draw_rounded_rect(x: f32, y: f32, w: f32, h: f32, r: f32, color: Color) {
    let r = r.min(w * 0.5).min(h * 0.5);

    // center
    draw_rectangle(x + r, y, w - 2.0 * r, h, color);
    draw_rectangle(x, y + r, r, h - 2.0 * r, color);
    draw_rectangle(x + w - r, y + r, r, h - 2.0 * r, color);

    // corners
    draw_circle(x + r, y + r, r, color);
    draw_circle(x + w - r, y + r, r, color);
    draw_circle(x + r, y + h - r, r, color);
    draw_circle(x + w - r, y + h - r, r, color);
}
