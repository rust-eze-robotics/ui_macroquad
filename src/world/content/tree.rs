use macroquad::experimental::animation::*;
use macroquad::prelude::*;
use macroquad::texture::Texture2D;

use crate::core::{Drawable, Vector};

pub struct Tree {
    pos: Vector,
    offset: Vector,
    image: Texture2D,
    sprite: AnimatedSprite,
}

impl Drawable for Tree {
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

impl Tree {
    pub async fn new(pos: Vector) -> Self {
        let mut ret = Self {
            pos,
            offset: Vector::new(0.0, 0.0),
            image: Texture2D::empty(),
            sprite: AnimatedSprite::new(
                192,
                192,
                &[
                    Animation {
                        name: "tree_0".to_string(),
                        row: 0,
                        frames: 4,
                        fps: 12,
                    },
                    Animation {
                        name: "tree_1".to_string(),
                        row: 1,
                        frames: 2,
                        fps: 12,
                    },
                ],
                true,
            ),
        };

        ret.image = load_texture("data/assets/contents/tree/tree.png")
            .await
            .unwrap();

        ret
    }
}
