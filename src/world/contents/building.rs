use macroquad::prelude::*;
use macroquad::texture::Texture2D;

pub struct Bulding {
    image: Texture2D,
}

impl Bulding {
    pub async fn load_texture(&mut self) {
        self.image = load_texture("data/assets/contents/building/building.png")
            .await
            .unwrap();
    }

    pub fn draw(&mut self) {
        draw_texture(&self.image, 0., 0., WHITE);
    }
}

impl Default for Bulding {
    fn default() -> Self {
        Self {
            image: Texture2D::empty(),
        }
    }
}
