use macroquad::prelude::*;
use macroquad::texture::Texture2D;

use crate::core::{is_in_window, Drawable};

use super::Content;

use std::rc::Rc;

pub struct None {
    pub(super) pos: Vec2,
    pub(super) offset: Vec2,
    pub(super) texture: Rc<Texture2D>,
}

impl Content for None {}

impl Drawable for None {
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
