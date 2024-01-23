use macroquad::experimental::animation::*;
use macroquad::prelude::*;
use macroquad::texture::Texture2D;

pub struct Water {
    sprite: AnimatedSprite,
    image: Texture2D,
}

impl Water {
    pub async fn load_texture(&mut self) {
        self.image = load_texture("data/assets/contents/water/water.png")
            .await
            .unwrap();
    }

    pub fn draw(&mut self) {
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

impl Default for Water {
    fn default() -> Self {
        Self {
            sprite: AnimatedSprite::new(
                192,
                192,
                &[
                    Animation {
                        name: "water_0".to_string(),
                        row: 0,
                        frames: 8,
                        fps: 12,
                    }
                ],
                true,
            ),
            image: Texture2D::empty(),
        }
    }
}
