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

const WALK_DURATION: Duration = Duration::from_millis(500);

enum State {
    Idle,
    Walk(Instant),
}

pub struct Robot {
    pub pos: Vec2,
    pub offset: Vec2,
    pub texture: Texture2D,
    pub sprite: AnimatedSprite,
    pub last_pos: Vec2,
    state: State,
}

impl Drawable for Robot {
    fn draw(&mut self, context: &Context) {
        if is_in_window(context, &self.pos, &self.offset, 192.0, 192.0) {
            match self.state {
                State::Idle => {
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
                State::Walk(instant) => {
                    draw_texture_ex(
                        &self.texture,
                        self.last_pos.x
                            + self.offset.x
                            + (self.pos.x - self.last_pos.x) * instant.elapsed().as_millis() as f32
                                / WALK_DURATION.as_millis() as f32,
                        self.last_pos.y
                            + self.offset.y
                            + (self.pos.y - self.last_pos.y) * instant.elapsed().as_millis() as f32
                                / WALK_DURATION.as_millis() as f32,
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
            last_pos: Vec2::new(col as f32 * TILE_WIDTH, row as f32 * TILE_WIDTH),
            state: State::Idle,
        }
    }

    pub fn update(&mut self, row: usize, col: usize) {
        self.last_pos = self.pos;
        self.pos.x = col as f32 * TILE_WIDTH;
        self.pos.y = row as f32 * TILE_WIDTH;
    }

    pub fn update_state(&mut self, timestamp: &mut Instant) {
        match self.state {
            State::Idle => {
                if self.pos != self.last_pos {
                    self.state = State::Walk(Instant::now());
                }
            }
            State::Walk(instant) => {
                if instant.elapsed() >= WALK_DURATION {
                    self.last_pos = self.pos;
                    self.state = State::Idle;
                }

                *timestamp = std::time::Instant::now();
            }
        }
    }

    pub fn get_target_pos(&self) -> Vec2 {
        match self.state {
            State::Idle => self.pos + self.offset + TILE_WIDTH / 2.0,
            State::Walk(instant) => Vec2::new(
                self.last_pos.x
                    + self.offset.x
                    + (self.pos.x - self.last_pos.x) * instant.elapsed().as_millis() as f32
                        / WALK_DURATION.as_millis() as f32
                    + TILE_WIDTH / 2.0,
                self.last_pos.y
                    + self.offset.y
                    + (self.pos.y - self.last_pos.y) * instant.elapsed().as_millis() as f32
                        / WALK_DURATION.as_millis() as f32
                    + TILE_WIDTH / 2.0,
            ),
        }
    }
}
