use macroquad::{
    color::Color,
    math::{Rect, Vec2},
    shapes::{draw_rectangle, draw_rectangle_lines},
};

pub struct Tile {
    pub pos: Vec2,
    pub face_rect: Rect,
    pub base_rect: Rect,
    pub face_c: Color,
    pub base_c: Color,
    pub border_c: Color,
}

impl Tile {
    pub fn new_input_tile(
        pos: Vec2,
        size: f32,
        face_c: Color,
        base_c: Color,
        border_c: Color,
    ) -> Self {
        let w = size;
        let h = size;
        let h_base = size / 5.;

        let x = pos.x * size;
        let y = pos.y * size + pos.y * h_base;
        let y_base = y + size;

        Self {
            pos,
            face_rect: Rect { x, y, w, h },
            base_rect: Rect {
                x,
                y: y_base,
                w,
                h: h_base,
            },
            face_c,
            base_c,
            border_c,
        }
    }

    pub fn render(&self) {
        let x = self.face_rect.x;
        let y = self.face_rect.y;
        let w = self.face_rect.w;
        let h = self.face_rect.h;
        draw_rectangle(x, y, w, h, self.face_c);

        let x = self.base_rect.x;
        let y = self.base_rect.y;
        let w = self.base_rect.w;
        let h = self.base_rect.h;
        draw_rectangle(x, y, w, h, self.base_c);

        let x = self.face_rect.x;
        let y = self.face_rect.y;
        let w = self.face_rect.w;
        let h = self.face_rect.h + self.base_rect.h;
        draw_rectangle_lines(x, y, w, h, 2., self.border_c);
    }
}
