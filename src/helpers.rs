use crate::{config::Z_LAYERS, cw_draws::Shape};

pub fn create_z_layer_vector() -> Vec<Vec<Shape>> {
    let mut z_layer: Vec<Vec<Shape>> = Vec::with_capacity(Z_LAYERS);
    for _ in 0..Z_LAYERS {
        z_layer.push(Vec::with_capacity(Z_LAYERS));
    }

    z_layer
}
