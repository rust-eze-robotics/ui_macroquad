use macroquad::prelude::*;
use macroquad::texture::Texture2D;

pub struct Chest {
    image: Texture2D,
}

impl Chest {
    pub async fn load_texture(&mut self) {
        self.image = load_texture("data/assets/contents/chest/chest.png")
            .await
            .unwrap();
    }

    pub fn draw(&mut self) {
        draw_texture(&self.image, 0., 0., WHITE);
    }
}

impl Default for Chest {
    fn default() -> Self {
        Self {
            image: Texture2D::empty(),
        }
    }
}
