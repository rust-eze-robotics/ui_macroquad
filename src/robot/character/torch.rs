use std::rc::Rc;

use macroquad::{
    experimental::animation::{AnimatedSprite, Animation},
    prelude::*,
};

use super::Character;

pub struct Torch {
    pub texture: Rc<Texture2D>,
}

impl Character for Torch {
    fn get_texture(&self) -> Rc<Texture2D> {
        self.texture.clone()
    }

    fn get_idle_sprite(&self) -> AnimatedSprite {
        AnimatedSprite::new(
            192,
            192,
            &[
                Animation {
                    name: "torch_0".to_string(),
                    row: 0,
                    frames: 7,
                    fps: 12,
                },
                Animation {
                    name: "torch_1".to_string(),
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
                name: "torch_2".to_string(),
                row: 2,
                frames: 6,
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
                name: "torch_4".to_string(),
                row: 4,
                frames: 6,
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
                name: "torch_3".to_string(),
                row: 3,
                frames: 6,
                fps: 12,
            }],
            true,
        )
    }
}
