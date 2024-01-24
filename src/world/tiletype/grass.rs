use macroquad::prelude::*;
use macroquad::texture::Texture2D;

use crate::core::Drawable;

use super::Tiletype;

pub struct Grass {
    pos: Vec2,
    offset: Vec2,
    image: Texture2D,
}

impl Tiletype for Grass {}

impl Drawable for Grass {
    fn draw(&mut self) {
        draw_texture(
            &self.image,
            self.pos.x + self.offset.x,
            self.pos.y + self.offset.y,
            WHITE,
        );
    }
}

impl Grass {
    pub async fn new(pos: Vec2) -> Self {
        let mut ret = Self {
            pos,
            offset: Vec2::new(0.0, 0.0),
            image: Texture2D::empty(),
        };

        ret.image = load_texture("data/assets/tiletypes/grass/grass.png")
            .await
            .unwrap();

        ret
    }
}
