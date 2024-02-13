use macroquad::prelude::*;

use crate::core::{context::Context, Drawable};

use super::Content;

pub struct None {
    pub(super) pos: Vec2,
    pub(super) offset: Vec2,
}

impl Content for None {}

impl Drawable for None {
    fn draw(&mut self, _context: &Context) {}
}
