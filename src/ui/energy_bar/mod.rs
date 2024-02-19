use std::{any::Any, fmt::Pointer, rc::Rc};

use macroquad::prelude::*;

use crate::{
    core::{context::Context, get_current_anchor_position, AnchorPosition, Drawable},
    robot::{
        character::{self, archer::Archer, CharacterEnum},
        Robot,
    },
};

use self::factory::BarFactory;

use super::UiItem;

pub mod factory;

pub struct EnergyBar {
    pos: Vec2,
    anchor_pos: AnchorPosition,
    texture: Rc<Texture2D>,
    bar_factory: BarFactory,
}

impl EnergyBar {
    pub async fn new() -> Self {
        Self {
            pos: Vec2::default(),
            anchor_pos: AnchorPosition::DownLeft(Vec2::new(0.0, -90.0)),
            texture: Rc::new(Texture2D::empty()),
            bar_factory: BarFactory::new().await,
        }
    }

    pub fn update_energy(&mut self, character: &CharacterEnum, energy: usize) {
        match character {
            CharacterEnum::Archer => self.texture = self.bar_factory.get_archer_texture(energy),
            CharacterEnum::Pawn => self.texture = self.bar_factory.get_pawn_texture(energy),
            CharacterEnum::Torch => self.texture = self.bar_factory.get_torch_texture(energy),
            CharacterEnum::Warrior => self.texture = self.bar_factory.get_warrior_texture(energy),
        }
    }
}

impl UiItem for EnergyBar {
    fn update_gui(&mut self) {
        self.pos = get_current_anchor_position(self.anchor_pos);
    }

    fn handle_input(&mut self) {}
}

impl Drawable for EnergyBar {
    fn draw(&mut self, _context: &Context) {
        draw_texture(&self.texture, self.pos.x, self.pos.y, LIGHTGRAY);
    }
}
