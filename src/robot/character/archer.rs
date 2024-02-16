use std::rc::Rc;

use macroquad::{
    experimental::animation::{AnimatedSprite, Animation},
    prelude::*,
};

use super::Character;

pub struct Archer {
    pub texture: Rc<Texture2D>,
}

impl Character for Archer {
    fn get_texture(&self) -> Rc<Texture2D> {
        self.texture.clone()
    }

    fn get_idle_sprite(&self) -> AnimatedSprite {
        AnimatedSprite::new(
            192,
            192,
            &[
                Animation {
                    name: "archer_0".to_string(),
                    row: 0,
                    frames: 6,
                    fps: 12,
                },
                Animation {
                    name: "archer_1".to_string(),
                    row: 1,
                    frames: 6,
                    fps: 12,
                },
            ],
            true,
        )
    }

    fn get_interact_right_sprite(&self) -> AnimatedSprite {
        AnimatedSprite::new(
            192,
            192,
            &[Animation {
                name: "archer_4".to_string(),
                row: 4,
                frames: 8,
                fps: 12,
            }],
            true,
        )
    }

    fn get_interact_up_sprite(&self) -> AnimatedSprite {
        AnimatedSprite::new(
            192,
            192,
            &[Animation {
                name: "archer_2".to_string(),
                row: 2,
                frames: 8,
                fps: 12,
            }],
            true,
        )
    }

    fn get_interact_down_sprite(&self) -> AnimatedSprite {
        AnimatedSprite::new(
            192,
            192,
            &[Animation {
                name: "archer_6".to_string(),
                row: 6,
                frames: 8,
                fps: 12,
            }],
            true,
        )
    }
}
