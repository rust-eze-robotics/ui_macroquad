use macroquad::prelude::*;
use macroquad::texture::Texture2D;

use crate::core::Drawable;

use super::Content;

pub struct Rock {
    pos: Vec2,
    offset: Vec2,
    image: Texture2D,
}

impl Content for Rock {}

impl Drawable for Rock {
    fn draw(&mut self) {
        draw_texture(&self.image, 0., 0., WHITE);
    }
}

impl Rock {
    pub async fn new(pos: Vec2) -> Self {
        let mut ret = Self {
            pos,
            offset: Vec2::new(0.0, 0.0),
            image: Texture2D::empty(),
        };

        ret.image = load_texture("data/assets/contents/rock/rock.png")
            .await
            .unwrap();

        ret
    }
}
