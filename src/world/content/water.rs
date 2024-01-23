use macroquad::experimental::animation::*;
use macroquad::prelude::*;
use macroquad::texture::Texture2D;

use crate::core::Drawable;

use super::Content;

pub struct Water {
    pos: Vec2,
    offset: Vec2,
    image: Texture2D,
    sprite: AnimatedSprite,
}

impl Content for Water {}

impl Drawable for Water {
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

impl Water {
    pub async fn new(pos: Vec2) -> Self {
        let mut ret = Self {
            pos,
            offset: Vec2::new(0.0, 0.0),
            image: Texture2D::empty(),
            sprite: AnimatedSprite::new(
                192,
                192,
                &[Animation {
                    name: "water_0".to_string(),
                    row: 0,
                    frames: 8,
                    fps: 12,
                }],
                true,
            ),
        };

        ret.image = load_texture("data/assets/contents/water/water.png")
            .await
            .unwrap();

        ret
    }
}
