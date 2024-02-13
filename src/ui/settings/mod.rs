use macroquad::prelude::*;

use crate::core::{context::Context, Drawable};

pub struct SettingsModal {
    pos: Vec2,
}

impl Drawable for SettingsModal {
    fn draw(&mut self, _context: &Context) {
        draw_rectangle(self.pos.x, self.pos.y, 192.0, 96.0, BEIGE);
        draw_rectangle_lines(self.pos.x, self.pos.y, 192.0, 96.0, 5.0, GOLD);
    }
}
