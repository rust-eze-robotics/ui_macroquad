use std::{cell::RefCell, collections::HashMap, rc::Rc, thread, time::Duration};

use macroquad::{
    experimental::animation::{AnimatedSprite, Animation},
    prelude::*,
};

use robotics_lib::{
    energy::Energy,
    event::events::Event,
    interface::{go, one_direction_view, robot_map, robot_view},
    runner::{backpack::BackPack, Robot as RobRobot, Runnable},
    world::{coordinates::Coordinate, World as RobWorld},
};

use crate::{
    core::{is_in_window, Drawable},
    world::{tiletype::Tiletype, World, TILE_WIDTH},
};

pub struct Robot {
    pub pos: Vec2,
    pub offset: Vec2,
    pub texture: Texture2D,
    pub sprite: AnimatedSprite,
}

impl Drawable for Robot {
    fn draw(&mut self, camera: &Camera2D) {
        if is_in_window(camera, &self.pos, &self.offset, 192.0, 192.0) {
            draw_texture_ex(
                &self.texture,
                self.pos.x + self.offset.x,
                self.pos.y + self.offset.y,
                LIGHTGRAY,
                DrawTextureParams {
                    source: Some(self.sprite.frame().source_rect),
                    dest_size: Some(self.sprite.frame().dest_size),
                    ..Default::default()
                },
            );
        }

        self.sprite.update();
    }
}

impl Robot {
    pub async fn new((row, col): (usize, usize)) -> Self {
        Self {
            pos: Vec2::new(col as f32 * TILE_WIDTH, row as f32 * TILE_WIDTH),
            offset: Vec2::new(0.0, 0.0),
            texture: load_texture("data/assets/robot/warrior.png").await.unwrap(),
            sprite: AnimatedSprite::new(
                192,
                192,
                &[
                    Animation {
                        name: "warrior_0".to_string(),
                        row: 0,
                        frames: 6,
                        fps: 12,
                    },
                    Animation {
                        name: "warrior_1".to_string(),
                        row: 1,
                        frames: 6,
                        fps: 12,
                    },
                ],
                true,
            ),
        }
    }

    pub fn update_pos(&mut self, row: usize, col: usize) {
        self.pos.x = col as f32 * TILE_WIDTH;
        self.pos.y = row as f32 * TILE_WIDTH;
    }
}
