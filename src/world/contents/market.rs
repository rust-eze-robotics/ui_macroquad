use macroquad::prelude::*;
use macroquad::texture::Texture2D;

pub struct Market {
    image: Texture2D,
}

impl Market {
    pub async fn load_texture(&mut self) {
        self.image = load_texture("data/assets/contents/market/market.png")
            .await
            .unwrap();
    }

    pub fn draw(&mut self) {
        draw_texture(&self.image, 0., 0., WHITE);
    }
}

impl Default for Market {
    fn default() -> Self {
        Self {
            image: Texture2D::empty(),
        }
    }
}
