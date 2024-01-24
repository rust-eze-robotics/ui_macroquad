use macroquad::experimental::animation::*;
use macroquad::prelude::*;
use macroquad::texture::Texture2D;

use crate::core::Drawable;

use super::Content;

pub struct Market {
    pos: Vec2,
    offset: Vec2,
    image: Texture2D,
    sprite: AnimatedSprite,
}

impl Content for Market {}

impl Drawable for Market {
    fn draw(&mut self) {
        draw_texture_ex(
            &self.image,
            self.pos.x + self.offset.x,
            self.pos.y + self.offset.y,
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
    pub async fn new(pos: Vec2) -> Self {
        let mut ret = Self {
            pos,
            offset: Vec2::new(0.0, 0.0),
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
