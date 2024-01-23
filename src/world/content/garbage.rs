use macroquad::prelude::*;
use macroquad::texture::Texture2D;

pub struct Garbage {
    image: Texture2D,
}

impl Garbage {
    pub async fn load_texture(&mut self) {
        self.image = load_texture("data/assets/contents/garbage/garbage.png")
            .await
            .unwrap();
    }

    pub fn draw(&mut self) {
        draw_texture(&self.image, 0., 0., WHITE);
    }
}

impl Default for Garbage {
    fn default() -> Self {
        Self {
            image: Texture2D::empty(),
        }
    }
}
