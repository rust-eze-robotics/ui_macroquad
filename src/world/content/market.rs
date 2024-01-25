use std::rc::Rc;

use macroquad::experimental::animation::*;
use macroquad::prelude::*;
use macroquad::texture::Texture2D;

use crate::core::{is_in_window, Drawable};

use super::Content;

pub struct Market {
    pub(super) pos: Vec2,
    pub(super) offset: Vec2,
    pub(super) texture: Rc<Texture2D>,
    pub(super) sprite: AnimatedSprite,
}

impl Content for Market {}

impl Drawable for Market {
    fn draw(&mut self, camera: &Camera2D) {
        if is_in_window(camera, &self.pos, &self.offset, 256.0, 192.0) {
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
