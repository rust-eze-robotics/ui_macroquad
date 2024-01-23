use macroquad::prelude::*;
use macroquad::texture::Texture2D;

use crate::core::{Drawable, Vector};

pub struct Scarecrow {
    pos: Vector,
    offset: Vector,
    image: Texture2D,
}

impl Drawable for Scarecrow {
    fn draw(&mut self) {
        draw_texture(&self.image, 0., 0., WHITE);
    }
}

impl Scarecrow {
    pub async fn new(pos: Vector) -> Self {
        let mut ret = Self {
            pos,
            offset: Vector::new(0.0, 0.0),
            image: Texture2D::empty(),
        };

        ret.image = load_texture("data/assets/contents/scarecrow/scarecrow.png")
            .await
            .unwrap();

        ret
    }
}
