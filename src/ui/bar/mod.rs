use macroquad::prelude::*;

use crate::core::{
    context::Context, get_current_anchor_position, AnchorPosition, Drawable, BAR_SIZE,
};

use super::UiItem;

pub struct Bar {
    pos: Vec2,
    anchor_pos: AnchorPosition,
    size: Vec2,
}

impl Bar {
    pub fn new() -> Self {
        Self {
            pos: Vec2::default(),
            anchor_pos: AnchorPosition::DownLeft(Vec2::new(0.0, BAR_SIZE.y * -1.0)),
            size: BAR_SIZE,
        }
    }
}

impl UiItem for Bar {
    fn update_gui(&mut self, _context: &Context) {
        self.pos = get_current_anchor_position(self.anchor_pos);
    }

    fn handle_input(&mut self, _context: &Context) {}
}

impl Drawable for Bar {
    fn draw(&mut self, context: &Context) {
        draw_rectangle_lines(self.pos.x, self.pos.y, self.size.x, self.size.y, 5.0, GOLD);
    }
}
