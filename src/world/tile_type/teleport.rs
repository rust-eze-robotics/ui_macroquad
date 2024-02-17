use macroquad::texture::Texture2D;
use macroquad::{experimental::animation::AnimatedSprite, prelude::*};

use crate::core::TILE_SIZE;
use crate::core::{context::Context, is_in_window, Drawable};

use super::TileType;

use std::rc::Rc;

pub struct Teleport {
    pub(super) pos: Vec2,
    pub(super) offset: Vec2,
    pub(super) texture: Rc<Texture2D>,
    pub(super) texture_ground: Rc<Texture2D>,
    pub(super) sprite: AnimatedSprite,
}

impl TileType for Teleport {}

impl Drawable for Teleport {
    fn draw(&mut self, context: &Context) {
        if is_in_window(
            context,
            &self.pos,
            &self.offset,
            self.texture_ground.size().x,
            self.texture_ground.size().y,
        ) {
            draw_texture(&self.texture_ground, self.pos.x, self.pos.y, LIGHTGRAY);

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
