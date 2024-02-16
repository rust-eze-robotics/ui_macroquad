use std::rc::Rc;

use macroquad::{
    experimental::animation::{AnimatedSprite, Animation},
    prelude::*,
};

use super::Character;

pub struct Pawn {
    pub texture: Rc<Texture2D>,
}

impl Character for Pawn {
    fn get_texture(&self) -> Rc<Texture2D> {
        self.texture.clone()
    }

    fn get_idle_sprite(&self) -> AnimatedSprite {
        AnimatedSprite::new(
            192,
            192,
            &[
                Animation {
                    name: "pawn_0".to_string(),
                    row: 0,
                    frames: 6,
                    fps: 12,
                },
                Animation {
                    name: "pawn_1".to_string(),
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
            &[
                Animation {
                    name: "pawn_2".to_string(),
                    row: 2,
                    frames: 6,
                    fps: 12,
                },
                Animation {
                    name: "pawn_3".to_string(),
                    row: 3,
                    frames: 6,
                    fps: 12,
                },
            ],
            true,
        )
    }

    fn get_interact_up_sprite(&self) -> AnimatedSprite {
        self.get_interact_right_sprite()
    }

    fn get_interact_down_sprite(&self) -> AnimatedSprite {
        self.get_interact_right_sprite()
    }
}
