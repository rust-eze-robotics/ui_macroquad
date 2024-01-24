use macroquad::prelude::*;
use macroquad::texture::Texture2D;

use crate::core::Drawable;

use super::Content;

pub struct Bank {
    pos: Vec2,
    offset: Vec2,
    texture: Texture2D,
}

impl Content for Bank {}

impl Drawable for Bank {
    fn draw(&mut self) {
        draw_texture(
            &self.texture,
            self.pos.x + self.offset.x,
            self.pos.y + self.offset.y,
            WHITE,
        );
    }
}

impl Bank {
    pub async fn new(pos: Vec2) -> Self {
        let mut ret = Self {
            pos,
            offset: Vec2::new(0.0, 0.0),
            texture: Texture2D::empty(),
        };

        ret.texture = load_texture("data/assets/contents/bank/bank_empty.png")
            .await
            .unwrap();

        ret
    }
}
