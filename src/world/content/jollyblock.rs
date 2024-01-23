use macroquad::experimental::animation::*;
use macroquad::prelude::*;
use macroquad::texture::Texture2D;

use crate::core::{Drawable, Vector};

pub struct Jollyblock {
    pos: Vector,
    offset: Vector,
    image: Texture2D,
    sprite: AnimatedSprite,
}

impl Drawable for Jollyblock {
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

impl Jollyblock {
    pub async fn new(pos: Vector) -> Self {
        let mut ret = Self {
            pos,
            offset: Vector::new(0.0, 0.0),
            image: Texture2D::empty(),
            sprite: AnimatedSprite::new(
                128,
                128,
                &[
                    Animation {
                        name: "jollyblock_0".to_string(),
                        row: 0,
                        frames: 8,
                        fps: 12,
                    },
                    Animation {
                        name: "jollyblock_1".to_string(),
                        row: 1,
                        frames: 6,
                        fps: 12,
                    },
                ],
                true,
            ),
        };

        ret.image = load_texture("data/assets/contents/jollyblock/jollyblock.png")
            .await
            .unwrap();

        ret
    }
}
