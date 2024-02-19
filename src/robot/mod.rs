use std::{
    rc::Rc,
    time::{Duration, Instant},
};

use macroquad::{experimental::animation::AnimatedSprite, prelude::*};
use robotics_lib::world::tile::Tile as RobTile;

use crate::{
    core::{context::Context, is_in_window, Drawable, TILE_SIZE},
    world::World,
};

use self::character::{factory::CharacterFactory, Character};

pub mod character;

pub enum RobotState {
    Init(Instant),
    Idle(Instant),
    Walk(Instant, Vec2),
    Teleport(Instant, Vec2),
    Interact(Instant, Vec2, RobTile),
}

pub struct Robot {
    pub pos: Vec2,
    pub offset: Vec2,
    pub orientation: bool,
    pub character: Box<dyn Character>,
    state: RobotState,
    texture: Rc<Texture2D>,
    sprite: AnimatedSprite,
    character_factory: CharacterFactory,
}

impl Robot {
    pub async fn new((row, col): (usize, usize)) -> Self {
        let character_factory = CharacterFactory::new().await;
        let character = Box::new(character_factory.new_warrior());

        Self {
            pos: Vec2::new(col as f32 * TILE_SIZE.x, row as f32 * TILE_SIZE.y),
            offset: Vec2::new(0.0, 0.0),
            orientation: false,
            state: RobotState::Init(Instant::now()),
            texture: character.get_texture(),
            sprite: character.get_init_sprite(),
            character,
            character_factory,
        }
    }

    pub fn set_idle(&mut self, context: &Context) {
        self.sprite = self.character.get_idle_sprite(context);
        self.state = RobotState::Idle(Instant::now());
    }

    pub fn set_walk(&mut self, context: &Context, pos: Vec2) {
        self.sprite = self.character.get_idle_sprite(context);
        self.state = RobotState::Walk(Instant::now(), pos);
    }

    pub fn set_teleport(&mut self, context: &Context, pos: Vec2) {
        self.sprite = self.character.get_idle_sprite(context);
        self.state = RobotState::Teleport(Instant::now(), pos);
    }

    pub fn set_interact(&mut self, context: &Context, pos: Vec2, tile: RobTile) {
        self.sprite = self.character.get_idle_sprite(context);

        if pos.y == self.pos.y {
            self.sprite = self.character.get_interact_right_sprite(context);
        } else if pos.y < self.pos.y {
            self.sprite = self.character.get_interact_up_sprite(context);
        } else {
            self.sprite = self.character.get_interact_down_sprite(context);
        }

        self.state = RobotState::Interact(Instant::now(), pos, tile);
    }

    pub fn set_archer(&mut self) {
        self.character = Box::new(self.character_factory.new_archer());
        self.texture = self.character.get_texture();
        self.sprite = self.character.get_init_sprite();
    }

    pub fn set_pawn(&mut self) {
        self.character = Box::new(self.character_factory.new_pawn());
        self.texture = self.character.get_texture();
        self.sprite = self.character.get_init_sprite();
    }

    pub fn set_torch(&mut self) {
        self.character = Box::new(self.character_factory.new_torch());
        self.texture = self.character.get_texture();
        self.sprite = self.character.get_init_sprite();
    }

    pub fn set_warrior(&mut self) {
        self.character = Box::new(self.character_factory.new_warrior());
        self.texture = self.character.get_texture();
        self.sprite = self.character.get_init_sprite();
    }

    pub fn is_ready(&self, context: &Context) -> bool {
        match self.state {
            RobotState::Idle(instant) => instant.elapsed() > context.tick_duration,
            _ => false,
        }
    }

    pub fn update_state(&mut self, context: &Context, world: &mut World) {
        match &self.state {
            RobotState::Init(instant) => {
                if instant.elapsed() > Duration::from_millis(500) {
                    self.set_idle(context);
                }
            }
            RobotState::Idle(_) => {}
            RobotState::Walk(instant, pos) => {
                if instant.elapsed() > context.tick_duration {
                    self.pos = *pos;
                    self.set_idle(context);
                }
            }
            RobotState::Teleport(instant, pos) => {
                if instant.elapsed() > context.tick_duration {
                    self.pos = *pos;
                    self.set_idle(context);
                }
            }
            RobotState::Interact(instant, pos, tile) => {
                if instant.elapsed() > context.tick_duration {
                    let (row, col) = (
                        (pos.y / TILE_SIZE.y) as usize,
                        (pos.x / TILE_SIZE.x) as usize,
                    );
                    world.update_tile(tile, (row, col));

                    self.set_idle(context);
                }
            }
        }
    }

    pub fn update_orientation(&mut self, new_pos: Vec2) {
        if new_pos.x < self.pos.x {
            self.orientation = true;
        } else if new_pos.x > self.pos.x {
            self.orientation = false;
        }
    }

    pub fn get_target_pos(&self, context: &Context) -> Vec2 {
        let ret = match self.state {
            RobotState::Init(_) | RobotState::Idle(_) | RobotState::Interact(_, _, _) => {
                Vec2::new(self.pos.x + self.offset.x, self.pos.y + self.offset.y)
            }
            RobotState::Walk(instant, pos) => Vec2::new(
                self.pos.x
                    + self.offset.x
                    + (pos.x - self.pos.x) * instant.elapsed().as_millis() as f32
                        / context.tick_duration.as_millis() as f32,
                self.pos.y
                    + self.offset.y
                    + (pos.y - self.pos.y) * instant.elapsed().as_millis() as f32
                        / context.tick_duration.as_millis() as f32,
            ),
            RobotState::Teleport(instant, pos) => {
                if instant.elapsed() < context.tick_duration / 2 {
                    Vec2::new(self.pos.x + self.offset.x, self.pos.y + self.offset.y)
                } else {
                    Vec2::new(pos.x + self.offset.x, pos.y + self.offset.y)
                }
            }
        };

        ret + Vec2::new(TILE_SIZE.x / 2.0, TILE_SIZE.y / 2.0)
    }
}

impl Drawable for Robot {
    fn draw(&mut self, context: &Context) {
        if is_in_window(context, &self.pos, &self.offset, 192.0, 192.0) {
            match self.state {
                RobotState::Init(_) | RobotState::Idle(_) | RobotState::Interact(_, _, _) => {
                    draw_texture_ex(
                        &self.texture,
                        self.pos.x + self.offset.x,
                        self.pos.y + self.offset.y,
                        LIGHTGRAY,
                        DrawTextureParams {
                            source: Some(self.sprite.frame().source_rect),
                            dest_size: Some(self.sprite.frame().dest_size),
                            flip_x: self.orientation,
                            ..Default::default()
                        },
                    );
                }
                RobotState::Walk(instant, pos) => {
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
                            flip_x: self.orientation,
                            ..Default::default()
                        },
                    );
                }
                RobotState::Teleport(instant, pos) => {
                    if instant.elapsed() < context.tick_duration / 2 {
                        draw_texture_ex(
                            &self.texture,
                            self.pos.x + self.offset.x,
                            self.pos.y + self.offset.y,
                            LIGHTGRAY,
                            DrawTextureParams {
                                source: Some(self.sprite.frame().source_rect),
                                dest_size: Some(self.sprite.frame().dest_size),
                                flip_x: self.orientation,
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
                                flip_x: self.orientation,
                                ..Default::default()
                            },
                        );
                    }
                }
            }
        }

        self.sprite.update();
    }
}
