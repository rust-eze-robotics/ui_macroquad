use macroquad::prelude::*;
use macroquad::texture::Texture2D;

use crate::core::{context::Context, Drawable};

use std::rc::Rc;

pub mod factory;

pub struct HorizontalBanner {
    pub pos: Vec2,
    pub texture_left: Rc<Texture2D>,
    pub texture_center: Rc<Texture2D>,
    pub texture_right: Rc<Texture2D>,
}

impl Drawable for HorizontalBanner {
    fn draw(&mut self, _context: &Context) {
        draw_rectangle(0.0, 0.0, 192.0, 96.0, BEIGE);
        draw_rectangle_lines(0.0, 0.0, 192.0, 96.0, 5.0, GOLD);
    }
}

pub struct VerticalBanner {
    pub pos: Vec2,
    pub texture_up: Rc<Texture2D>,
    pub texture_center: Rc<Texture2D>,
    pub texture_down: Rc<Texture2D>,
}
