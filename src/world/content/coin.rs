use macroquad::prelude::*;
use macroquad::texture::Texture2D;

use crate::core::Drawable;

use super::Content;

pub struct Coin {
    pos: Vec2,
    offset: Vec2,
    image: Texture2D,
}

impl Content for Coin {}

impl Drawable for Coin {
    fn draw(&mut self) {
        draw_texture(&self.image, 0., 0., WHITE);
    }
}

impl Coin {
    pub async fn new(pos: Vec2) -> Self {
        let mut ret = Self {
            pos,
            offset: Vec2::new(0.0, 0.0),
            image: Texture2D::empty(),
        };

        ret.image = load_texture("data/assets/contents/coin/coin.png")
            .await
            .unwrap();

        ret
    }
}
