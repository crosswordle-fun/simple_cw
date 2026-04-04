use macroquad::{
    color::Color,
    math::{Rect, Vec2},
    shapes::{draw_rectangle, draw_rectangle_lines},
    text::draw_text,
};

#[derive(Clone, Copy)]
pub struct Tile {
    pub pos: Vec2,
    pub face_rect: Rect,
    pub base_height: f32,
    pub border_size: f32,
    pub face_c: Color,
    pub base_c: Color,
    pub border_c: Color,
    pub letter: Option<char>,
}

impl Tile {
    pub fn new_input_tile(
        pos: Vec2,
        size: f32,
        border_size: f32,
        face_c: Color,
        base_c: Color,
        border_c: Color,
        letter: Option<char>,
    ) -> Self {
        let w = size - border_size * 2.;
        let h = w;
        let base_height = size / 5.;

        let x = pos.x * size + border_size;
        let y = pos.y * (size + base_height);

        Self {
            pos,
            face_rect: Rect { x, y, w, h },
            base_height,
            border_size,
            face_c,
            base_c,
            border_c,
            letter,
        }
    }

    pub fn render(&self) {
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

        if self.letter.is_some() {
            draw_text(
                self.letter.unwrap().to_string().as_str(),
                x + self.face_rect.w * 0.25,
                y + self.face_rect.w * 0.75,
                self.face_rect.w,
                self.base_c,
            );
        }
    }
}
