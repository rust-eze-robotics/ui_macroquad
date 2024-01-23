use macroquad::experimental::animation::*;
use macroquad::prelude::*;
use macroquad::texture::Texture2D;

pub struct Bank {
    image: Texture2D,
}

impl Bank {
    pub async fn load_texture(&mut self) {
        self.image = load_texture("data/assets/contents/bank/bank_empty.png")
            .await
            .unwrap();
    }

    pub fn draw(&mut self) {
        draw_texture(&self.image, 0., 0., WHITE);
    }
}

impl Default for Bank {
    fn default() -> Self {
        Self {
            image: Texture2D::empty(),
        }
    }
}
