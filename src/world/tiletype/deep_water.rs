use macroquad::prelude::*;
use macroquad::texture::Texture2D;

use crate::core::{Drawable, Vector};

pub struct DeepWater {
    pos: Vector,
    offset: Vector,
    image: Texture2D,
}

impl Drawable for DeepWater {
    fn draw(&mut self) {
        draw_texture(&self.image, 0., 0., WHITE);
    }
}

impl DeepWater {
    pub async fn new(pos: Vector) -> Self {
        let mut ret = Self {
            pos,
            offset: Vector::new(0.0, 0.0),
            image: Texture2D::empty(),
        };

        ret.image = load_texture("data/assets/tiletypes/deep_water/deep_water.png")
            .await
            .unwrap();

        ret
    }
}
