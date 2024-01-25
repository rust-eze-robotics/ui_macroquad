use macroquad::prelude::*;
use macroquad::texture::Texture2D;

use crate::core::{is_in_window, Drawable};

use super::Tiletype;

use std::rc::Rc;

pub struct Hill {
    pub(super) pos: Vec2,
    pub(super) offset: Vec2,
    pub(super) texture: Rc<Texture2D>,
}

impl Tiletype for Hill {}

impl Drawable for Hill {
    fn draw(&mut self, camera: &Camera2D) {
        if is_in_window(
            camera,
            &self.pos,
            &self.offset,
            self.texture.width(),
            self.texture.height(),
        ) {
            draw_texture(
                &self.texture,
                self.pos.x + self.offset.x,
                self.pos.y + self.offset.y,
                LIGHTGRAY,
            );
        }
    }
}
