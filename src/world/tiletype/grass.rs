use macroquad::prelude::*;
use macroquad::texture::Texture2D;

pub struct Grass {
    image: Texture2D,
}

impl Grass {
    pub async fn load_texture(&mut self) {
        self.image = load_texture("data/assets/tiletypes/grass/grass.png")
            .await
            .unwrap();
    }

    pub fn draw(&mut self) {
        draw_texture(&self.image, 0., 0., WHITE);
    }
}

impl Default for Grass {
    fn default() -> Self {
        Self {
            image: Texture2D::empty(),
        }
    }
}
