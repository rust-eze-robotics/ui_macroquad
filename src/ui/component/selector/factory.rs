use std::rc::Rc;

use macroquad::prelude::*;

use crate::robot::character::{factory::CharacterFactory, Character};

use super::{Selector, SelectorState};

pub struct SelectorFactory {
    character_factory: CharacterFactory,
}

impl SelectorFactory {
    pub async fn new() -> Self {
        Self {
            character_factory: CharacterFactory::new().await,
        }
    }

    pub fn new_archer_selector(&self) -> Selector {
        let character = self.character_factory.new_archer();
        let sprite = character.get_selector_idle_sprite();

        Selector {
            pos: Vec2::default(),
            size: sprite.frame().dest_size,
            character: Rc::new(character),
            sprite,
            state: SelectorState::Idle,
        }
    }

    pub fn new_pawn_selector(&self) -> Selector {
        let character = self.character_factory.new_pawn();
        let sprite = character.get_selector_idle_sprite();

        Selector {
            pos: Vec2::default(),
            size: sprite.frame().dest_size,
            character: Rc::new(character),
            sprite,
            state: SelectorState::Idle,
        }
    }

    pub fn new_torch_selector(&self) -> Selector {
        let character = self.character_factory.new_torch();
        let sprite = character.get_selector_idle_sprite();

        Selector {
            pos: Vec2::default(),
            size: sprite.frame().dest_size,
            character: Rc::new(character),
            sprite,
            state: SelectorState::Idle,
        }
    }

    pub fn new_warrior_selector(&self) -> Selector {
        let character = self.character_factory.new_warrior();
        let sprite = character.get_selector_idle_sprite();

        Selector {
            pos: Vec2::default(),
            size: sprite.frame().dest_size,
            character: Rc::new(character),
            sprite,
            state: SelectorState::Idle,
        }
    }
}
