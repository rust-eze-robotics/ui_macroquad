use std::{collections::HashMap, rc::Rc};

use macroquad::prelude::*;

use super::{archer::Archer, pawn::Pawn, torch::Torch, warrior::Warrior};

const ARCHER_ID: u8 = 0;
const PAWN_ID: u8 = 1;
const TORCH_ID: u8 = 2;
const WARRIOR_ID: u8 = 3;

pub struct CharacterFactory {
    textures: HashMap<u8, Rc<Texture2D>>,
}

impl CharacterFactory {
    pub async fn new() -> Self {
        let mut textures = HashMap::new();

        textures.insert(
            ARCHER_ID,
            Rc::new(
                load_texture("assets/textures/robot/archer.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            PAWN_ID,
            Rc::new(
                load_texture("assets/textures/robot/pawn.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            TORCH_ID,
            Rc::new(
                load_texture("assets/textures/robot/torch.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            WARRIOR_ID,
            Rc::new(
                load_texture("assets/textures/robot/warrior.png")
                    .await
                    .unwrap(),
            ),
        );

        Self { textures }
    }

    pub fn new_archer(&self) -> Archer {
        Archer {
            texture: self.textures[&ARCHER_ID].clone(),
        }
    }

    pub fn new_pawn(&self) -> Pawn {
        Pawn {
            texture: self.textures[&PAWN_ID].clone(),
        }
    }

    pub fn new_torch(&self) -> Torch {
        Torch {
            texture: self.textures[&TORCH_ID].clone(),
        }
    }

    pub fn new_warrior(&self) -> Warrior {
        Warrior {
            texture: self.textures[&WARRIOR_ID].clone(),
        }
    }
}
