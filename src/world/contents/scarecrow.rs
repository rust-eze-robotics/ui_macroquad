use macroquad::prelude::*;
use macroquad::texture::Texture2D;

pub struct Scarecrow {
    image: Texture2D,
}

impl Scarecrow {
    pub async fn load_texture(&mut self) {
        self.image = load_texture("data/assets/contents/scarecrow/scarecrow.png")
            .await
            .unwrap();
    }

    pub fn draw(&mut self) {
        draw_texture(&self.image, 0., 0., WHITE);
    }
}

impl Default for Scarecrow {
    fn default() -> Self {
        Self {
            image: Texture2D::empty(),
        }
    }
}
