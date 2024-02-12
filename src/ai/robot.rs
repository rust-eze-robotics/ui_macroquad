use std::time::{Duration, Instant};

use macroquad::{
    experimental::animation::{AnimatedSprite, Animation},
    prelude::*,
};

use crate::{
    context::Context,
    core::{is_in_window, Drawable},
    world::TILE_WIDTH,
};

pub enum State {
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
    pub state: State,
    pub energy: usize,
}

impl Drawable for Robot {
    fn draw(&mut self, context: &Context) {
        if is_in_window(context, &self.pos, &self.offset, 192.0, 192.0) {
            match self.state {
                State::Idle(_) => {
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
                State::Walking(instant, pos) => {
                    draw_texture_ex(
                        &self.texture,
                        self.pos.x
                            + self.offset.x
                            + (pos.x - self.pos.x) * instant.elapsed().as_millis() as f32
                                / context.clock_duration.as_millis() as f32,
                        self.pos.y
                            + self.offset.y
                            + (pos.y - self.pos.y) * instant.elapsed().as_millis() as f32
                                / context.clock_duration.as_millis() as f32,
                        LIGHTGRAY,
                        DrawTextureParams {
                            source: Some(self.sprite.frame().source_rect),
                            dest_size: Some(self.sprite.frame().dest_size),
                            ..Default::default()
                        },
                    );
                }
                State::Teleporting(instant, pos) => {
                    if instant.elapsed() < context.clock_duration / 2 {
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
                State::Interacting(_, _) => {
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
            state: State::Idle(Instant::now()),
            energy: 1000,
        }
    }

    pub fn ready(&self, context: &Context) -> bool {
        match self.state {
            State::Idle(instant) => instant.elapsed() > context.clock_duration,
            _ => false,
        }
    }

    pub fn update_state(&mut self, context: &Context) {
        match self.state {
            State::Idle(_) => {}
            State::Walking(instant, pos) => {
                if instant.elapsed() > context.clock_duration {
                    self.pos = pos;
                    self.state = State::Idle(Instant::now());
                }
            }
            State::Teleporting(instant, pos) => {
                if instant.elapsed() > context.clock_duration {
                    self.pos = pos;
                    self.state = State::Idle(Instant::now());
                }
            }
            State::Interacting(instant, _) => {
                if instant.elapsed() > context.clock_duration {
                    self.state = State::Idle(Instant::now());
                }
            }
        }
    }

    pub fn get_target_pos(&self, context: &Context) -> Vec2 {
        match self.state {
            State::Idle(_) => Vec2::new(self.pos.x + self.offset.x, self.pos.y + self.offset.y),
            State::Walking(instant, pos) => Vec2::new(
                self.pos.x
                    + self.offset.x
                    + (pos.x - self.pos.x) * instant.elapsed().as_millis() as f32
                        / context.clock_duration.as_millis() as f32,
                self.pos.y
                    + self.offset.y
                    + (pos.y - self.pos.y) * instant.elapsed().as_millis() as f32
                        / context.clock_duration.as_millis() as f32,
            ),
            State::Teleporting(instant, pos) => {
                if instant.elapsed() < context.clock_duration / 2 {
                    Vec2::new(self.pos.x + self.offset.x, self.pos.y + self.offset.y)
                } else {
                    Vec2::new(pos.x + self.offset.x, pos.y + self.offset.y)
                }
            }
            State::Interacting(_, _) => {
                Vec2::new(self.pos.x + self.offset.x, self.pos.y + self.offset.y)
            }
        }
    }
}
