use macroquad::prelude::*;

use crate::tile::Tile;

#[derive(Clone, Debug, Default)]
pub struct WordleInputBoard {
    pub face_rect: Rect,
    pub base_height: f32,
    pub border_size: f32,
    pub face_c: Color,
    pub base_c: Color,
    pub border_c: Color,
    pub slots: Vec<Tile>,
}

impl WordleInputBoard {
    pub fn new() -> Self {
        let screen_x = screen_width();
        let screen_y = screen_height();
        let x_percent = 0.5;
        let x = (screen_x - x_percent * screen_x) / 2.;
        let y = 0.80 * screen_y;
        let w = screen_x * x_percent;
        let base_height = y / 25.;
        let h = screen_y - y - base_height;
        let border_size = 2.;
        let face_c = GRAY;
        let base_c = DARKGRAY;
        let border_c = BLACK;

        let num_tiles_in_row = 11;
        let grid_size = screen_width() / num_tiles_in_row as f32;
        let start_idx = num_tiles_in_row / 2 - 2;
        let end_idx = num_tiles_in_row / 2 + 3;

        let mut slots = Vec::new();
        for i in start_idx..end_idx {
            let pos = Vec2::new(i as f32, 4.);
            slots.push(Tile::new_input_tile(
                pos,
                grid_size,
                border_size,
                face_c,
                face_c,
                border_c,
                None,
            ));
        }

        Self {
            face_rect: Rect { x, y, w, h },
            base_height,
            border_size,
            face_c,
            base_c,
            border_c,
            slots,
        }
    }

    pub fn render(&self) {
        self.render_board();
        // self.render_slots();
    }

    // fn render_slots(&self) {
    //     for slot in &self.slots {
    //         slot.render();
    //     }
    // }

    fn render_board(&self) {
        let x = self.face_rect.x;
        let y = self.face_rect.y;
        let w = self.face_rect.w;
        let h = self.face_rect.h;
        draw_rectangle(x, y, w, h, self.face_c);

        let y = self.face_rect.y + self.face_rect.h;
        let h = self.base_height;
        draw_rectangle(x, y, w, h, self.base_c);

        let y = self.face_rect.y;
        let h = self.face_rect.h + self.base_height;
        draw_rectangle_lines(x, y, w, h, self.border_size, self.border_c);
    }
}
