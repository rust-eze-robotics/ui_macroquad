use macroquad::prelude::*;
use macroquad::texture::Texture2D;

use crate::{
    context::Context,
    core::{is_in_window, Drawable},
};

use super::Tiletype;

use std::rc::Rc;

pub struct Lava {
    pub(super) pos: Vec2,
    pub(super) offset: Vec2,
    pub(super) texture: Rc<Texture2D>,
}

impl Tiletype for Lava {}

impl Drawable for Lava {
    fn draw(&mut self, context: &Context) {
        if is_in_window(
            context,
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
