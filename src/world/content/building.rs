use macroquad::prelude::*;
use macroquad::texture::Texture2D;

pub struct Building {
    image: Texture2D,
}

impl Building {
    pub async fn load_texture(&mut self) {
        self.image = load_texture("data/assets/contents/building/building.png")
            .await
            .unwrap();
    }

    pub fn draw(&mut self) {
        draw_texture(&self.image, 0., 0., WHITE);
    }
}

impl Default for Building {
    fn default() -> Self {
        Self {
            image: Texture2D::empty(),
        }
    }
}
