use macroquad::prelude::*;
use macroquad::texture::Texture2D;

use crate::core::Drawable;

use super::Tiletype;

pub struct Lava {
    pos: Vec2,
    offset: Vec2,
    image: Texture2D,
}

impl Tiletype for Lava {}

impl Drawable for Lava {
    fn draw(&mut self) {
        draw_texture(&self.image, 0., 0., WHITE);
    }
}

impl Lava {
    pub async fn new(pos: Vec2) -> Self {
        let mut ret = Self {
            pos,
            offset: Vec2::new(0.0, 0.0),
            image: Texture2D::empty(),
        };

        ret.image = load_texture("data/assets/tiletypes/lava/lava.png")
            .await
            .unwrap();

        ret
    }
}
