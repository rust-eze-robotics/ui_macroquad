use macroquad::experimental::animation::*;
use macroquad::prelude::*;
use macroquad::texture::Texture2D;

use crate::core::{Drawable, Vector};

pub struct Market {
    pos: Vector,
    offset: Vector,
    image: Texture2D,
    sprite: AnimatedSprite,
}

impl Drawable for Market {
    fn draw(&mut self) {
        draw_texture_ex(
            &self.image,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                source: Some(self.sprite.frame().source_rect),
                dest_size: Some(self.sprite.frame().dest_size),
                ..Default::default()
            },
        );

        self.sprite.update();
    }
}

impl Market {
    pub async fn new(pos: Vector) -> Self {
        let mut ret = Self {
            pos,
            offset: Vector::new(0.0, 0.0),
            image: Texture2D::empty(),
            sprite: AnimatedSprite::new(
                256,
                192,
                &[Animation {
                    name: "market_0".to_string(),
                    row: 0,
                    frames: 4,
                    fps: 12,
                }],
                true,
            ),
        };

        ret.image = load_texture("data/assets/contents/market/market.png")
            .await
            .unwrap();

        ret
    }
}
