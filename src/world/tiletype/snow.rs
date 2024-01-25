use std::rc::Rc;

use macroquad::prelude::*;
use macroquad::texture::Texture2D;

use crate::core::Drawable;

use super::Tiletype;

pub struct Snow {
    pub(super) pos: Vec2,
    pub(super) offset: Vec2,
    pub(super) texture: Rc<Texture2D>,
}

impl Tiletype for Snow {}

impl Drawable for Snow {
    fn draw(&mut self) {
        draw_texture(
            &self.texture,
            self.pos.x + self.offset.x,
            self.pos.y + self.offset.y,
            LIGHTGRAY,
        );
    }
}
