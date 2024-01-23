use macroquad::prelude::*;
use macroquad::texture::Texture2D;

use crate::core::Drawable;

pub struct Bank {
    image: Texture2D,
}

impl Drawable for Bank {
    fn draw(&mut self) {
        draw_texture(&self.image, 0., 0., WHITE);
    }
}

impl Bank {
    pub async fn default() -> Self {
        let mut ret = Self {
            image: Texture2D::empty(),
        };

        ret.image = load_texture("data/assets/contents/bank/bank_empty.png")
            .await
            .unwrap();

        ret
    }
}
