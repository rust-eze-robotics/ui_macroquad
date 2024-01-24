use macroquad::experimental::animation::*;
use macroquad::prelude::*;
use macroquad::texture::Texture2D;

use crate::core::Drawable;

use super::Content;

pub struct Fire {
    pos: Vec2,
    offset: Vec2,
    image: Texture2D,
    sprite: AnimatedSprite,
}

impl Content for Fire {}

impl Drawable for Fire {
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

impl Fire {
    pub async fn new(pos: Vec2) -> Self {
        let mut ret = Self {
            pos,
            offset: Vec2::new(0.0, 0.0),
            image: Texture2D::empty(),
            sprite: AnimatedSprite::new(
                128,
                128,
                &[Animation {
                    name: "fire_0".to_string(),
                    row: 0,
                    frames: 7,
                    fps: 12,
                }],
                true,
            ),
        };

        ret.image = load_texture("data/assets/contents/fire/fire.png")
            .await
            .unwrap();

        ret
    }
}
