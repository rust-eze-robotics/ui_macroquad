use std::rc::Rc;

use macroquad::{
    experimental::animation::{AnimatedSprite, Animation},
    prelude::*,
};

use crate::core::{context::Context, TICK_DURATION_DEFAULT};

use super::Character;

pub struct Archer {
    pub texture: Rc<Texture2D>,
}

impl Character for Archer {
    fn get_texture(&self) -> Rc<Texture2D> {
        self.texture.clone()
    }

    fn get_init_sprite(&self) -> AnimatedSprite {
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

    fn get_idle_sprite(&self, context: &Context) -> AnimatedSprite {
        AnimatedSprite::new(
            192,
            192,
            &[
                Animation {
                    name: "archer_0".to_string(),
                    row: 0,
                    frames: 6,
                    fps: 12 * TICK_DURATION_DEFAULT.as_millis() as u32
                        / context.tick_duration.as_millis() as u32,
                },
                Animation {
                    name: "archer_1".to_string(),
                    row: 1,
                    frames: 6,
                    fps: 12 * TICK_DURATION_DEFAULT.as_millis() as u32
                        / context.tick_duration.as_millis() as u32,
                },
            ],
            true,
        )
    }

    fn get_interact_right_sprite(&self, context: &Context) -> AnimatedSprite {
        AnimatedSprite::new(
            192,
            192,
            &[Animation {
                name: "archer_4".to_string(),
                row: 4,
                frames: 8,
                fps: 12 * TICK_DURATION_DEFAULT.as_millis() as u32
                    / context.tick_duration.as_millis() as u32,
            }],
            true,
        )
    }

    fn get_interact_up_sprite(&self, context: &Context) -> AnimatedSprite {
        AnimatedSprite::new(
            192,
            192,
            &[Animation {
                name: "archer_2".to_string(),
                row: 2,
                frames: 8,
                fps: 12 * TICK_DURATION_DEFAULT.as_millis() as u32
                    / context.tick_duration.as_millis() as u32,
            }],
            true,
        )
    }

    fn get_interact_down_sprite(&self, context: &Context) -> AnimatedSprite {
        AnimatedSprite::new(
            192,
            192,
            &[Animation {
                name: "archer_6".to_string(),
                row: 6,
                frames: 8,
                fps: 12 * TICK_DURATION_DEFAULT.as_millis() as u32
                    / context.tick_duration.as_millis() as u32,
            }],
            true,
        )
    }

    fn get_selector_idle_sprite(&self) -> AnimatedSprite {
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

    fn get_selector_selected_sprite(&self) -> AnimatedSprite {
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
}
