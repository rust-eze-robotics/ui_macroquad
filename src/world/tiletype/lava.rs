use macroquad::prelude::*;
use macroquad::texture::Texture2D;

use crate::core::{Drawable, Vector};

pub struct Lava {
    pos: Vector,
    offset: Vector,
    image: Texture2D,
}

impl Drawable for Lava {
    fn draw(&mut self) {
        draw_texture(&self.image, 0., 0., WHITE);
    }
}

impl Lava {
    pub async fn new(pos: Vector) -> Self {
        let mut ret = Self {
            pos,
            offset: Vector::new(0.0, 0.0),
            image: Texture2D::empty(),
        };

        ret.image = load_texture("data/assets/tiletypes/lava/lava.png")
            .await
            .unwrap();

        ret
    }
}
