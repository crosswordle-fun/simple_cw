use macroquad::prelude::*;

pub enum Shape {
    Rectangle(Rect, Color),
}

pub fn draw_wordle_input(z_layer: &mut Vec<Vec<Shape>>) {
    let grid_factor = 11.;
    let grid_size_x = screen_width() / grid_factor;

    let tile_padding = grid_size_x / 10.;
    let tile_size = grid_size_x - tile_padding;

    let x_total = 5. * tile_size + 4. * tile_padding;
    let x_rem = screen_width() - x_total;

    let x = x_rem / 2. - tile_padding;
    let y = screen_height() - grid_size_x - tile_padding;
    let w = x_total + tile_padding * 2.;
    let h = grid_size_x + tile_padding;
    let base = Shape::Rectangle(Rect { x, y, w, h }, WHITE);
    z_layer[0].push(base);

    for i in 0..5 {
        let x = i as f32 * (tile_size + tile_padding) + x_rem / 2.;
        let y = screen_height() - tile_padding - tile_size;
        let w = tile_size;
        let h = tile_size;
        let tile = Shape::Rectangle(Rect { x, y, w, h }, BLACK);

        z_layer[1].push(tile);
    }
}
