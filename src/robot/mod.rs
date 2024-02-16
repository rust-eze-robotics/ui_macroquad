use std::time::Instant;

use macroquad::{
    experimental::animation::{AnimatedSprite, Animation},
    prelude::*,
};

use crate::core::{context::Context, is_in_window, Drawable, TILE_SIZE};

pub enum RobotState {
    Idle(Instant),
    Walking(Instant, Vec2),
    Teleporting(Instant, Vec2),
    Interacting(Instant, Vec2),
}

pub struct Robot {
    pub pos: Vec2,
    pub offset: Vec2,
    pub texture: Texture2D,
    pub sprite: AnimatedSprite,
    pub state: RobotState,
    pub energy: usize,
}

impl Drawable for Robot {
    fn draw(&mut self, context: &Context) {
        if is_in_window(context, &self.pos, &self.offset, 192.0, 192.0) {
            match self.state {
                RobotState::Idle(_) => {
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
                RobotState::Walking(instant, pos) => {
                    draw_texture_ex(
                        &self.texture,
                        self.pos.x
                            + self.offset.x
                            + (pos.x - self.pos.x) * instant.elapsed().as_millis() as f32
                                / context.tick_duration.as_millis() as f32,
                        self.pos.y
                            + self.offset.y
                            + (pos.y - self.pos.y) * instant.elapsed().as_millis() as f32
                                / context.tick_duration.as_millis() as f32,
                        LIGHTGRAY,
                        DrawTextureParams {
                            source: Some(self.sprite.frame().source_rect),
                            dest_size: Some(self.sprite.frame().dest_size),
                            ..Default::default()
                        },
                    );
                }
                RobotState::Teleporting(instant, pos) => {
                    if instant.elapsed() < context.tick_duration / 2 {
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
                    } else {
                        draw_texture_ex(
                            &self.texture,
                            pos.x + self.offset.x,
                            pos.y + self.offset.y,
                            LIGHTGRAY,
                            DrawTextureParams {
                                source: Some(self.sprite.frame().source_rect),
                                dest_size: Some(self.sprite.frame().dest_size),
                                ..Default::default()
                            },
                        );
                    }
                }
                RobotState::Interacting(_, _) => {
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
            }
        }

        self.sprite.update();
    }
}

impl Robot {
    pub async fn new((row, col): (usize, usize)) -> Self {
        Self {
            pos: Vec2::new(col as f32 * TILE_SIZE.x, row as f32 * TILE_SIZE.y),
            offset: Vec2::new(0.0, 0.0),
            texture: load_texture("assets/textures/robot/warrior.png")
                .await
                .unwrap(),
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
            state: RobotState::Idle(Instant::now()),
            energy: 1000,
        }
    }

    pub fn ready(&self, context: &Context) -> bool {
        match self.state {
            RobotState::Idle(instant) => instant.elapsed() > context.tick_duration,
            _ => false,
        }
    }

    pub fn update_state(&mut self, context: &Context) {
        match self.state {
            RobotState::Idle(_) => {}
            RobotState::Walking(instant, pos) => {
                if instant.elapsed() > context.tick_duration {
                    self.pos = pos;
                    self.state = RobotState::Idle(Instant::now());
                }
            }
            RobotState::Teleporting(instant, pos) => {
                if instant.elapsed() > context.tick_duration {
                    self.pos = pos;
                    self.state = RobotState::Idle(Instant::now());
                }
            }
            RobotState::Interacting(instant, _) => {
                if instant.elapsed() > context.tick_duration {
                    self.state = RobotState::Idle(Instant::now());
                }
            }
        }
    }

    pub fn get_target_pos(&self, context: &Context) -> Vec2 {
        let ret = match self.state {
            RobotState::Idle(_) => {
                Vec2::new(self.pos.x + self.offset.x, self.pos.y + self.offset.y)
            }
            RobotState::Walking(instant, pos) => Vec2::new(
                self.pos.x
                    + self.offset.x
                    + (pos.x - self.pos.x) * instant.elapsed().as_millis() as f32
                        / context.tick_duration.as_millis() as f32,
                self.pos.y
                    + self.offset.y
                    + (pos.y - self.pos.y) * instant.elapsed().as_millis() as f32
                        / context.tick_duration.as_millis() as f32,
            ),
            RobotState::Teleporting(instant, pos) => {
                if instant.elapsed() < context.tick_duration / 2 {
                    Vec2::new(self.pos.x + self.offset.x, self.pos.y + self.offset.y)
                } else {
                    Vec2::new(pos.x + self.offset.x, pos.y + self.offset.y)
                }
            }
            RobotState::Interacting(_, _) => {
                Vec2::new(self.pos.x + self.offset.x, self.pos.y + self.offset.y)
            }
        };

        ret + Vec2::new(TILE_SIZE.x / 2.0, TILE_SIZE.y / 2.0)
    }
}
