use std::rc::Rc;

use macroquad::experimental::animation::*;
use macroquad::prelude::*;
use macroquad::texture::Texture2D;

use crate::{
    context::Context,
    core::{is_in_window, Drawable},
};

use super::Content;

pub struct Tree {
    pub(super) pos: Vec2,
    pub(super) offset: Vec2,
    pub(super) texture: Rc<Texture2D>,
    pub(super) sprite: AnimatedSprite,
}

impl Content for Tree {}

impl Drawable for Tree {
    fn draw(&mut self, context: &Context) {
        if is_in_window(context, &self.pos, &self.offset, 192.0, 192.0) {
            draw_texture_ex(
                &self.texture,
                self.pos.x + self.offset.x,
                self.pos.y + self.offset.y,
                LIGHTGRAY,
                DrawTextureParams {
                    source: Some(self.sprite.frame().source_rect),
                    dest_size: Some(self.sprite.frame().dest_size),
                    ..Default::default()
                },
            );
        }

        self.sprite.update();
    }
}
