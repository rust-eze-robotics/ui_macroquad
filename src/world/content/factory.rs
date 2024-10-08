use std::{collections::HashMap, rc::Rc};

use macroquad::{
    experimental::animation::{AnimatedSprite, Animation},
    prelude::*,
};
use robotics_lib::world::tile::Content as RobContent;

use super::{
    bank::Bank, bin::Bin, building::Building, bush::Bush, chest::Chest, coin::Coin, fire::Fire,
    fish::Fish, garbage::Garbage, jollyblock::Jollyblock, market::Market, none::None, rock::Rock,
    scarecrow::Scarecrow, tree::Tree, water::Water, Content,
};

const BANK_ID: u8 = 0;
const BIN_ID: u8 = 1;
const BUILDING_ID: u8 = 2;
const BUSH_ID: u8 = 3;
const CHEST_ID: u8 = 4;
const COIN_ID: u8 = 5;
const FIRE_ID: u8 = 6;
const FISH_ID: u8 = 7;
const GARBAGE_ID: u8 = 8;
const JOLLYBLOCK_ID: u8 = 9;
const MARKET_ID: u8 = 10;
const NONE_ID: u8 = 11;
const ROCK_ID: u8 = 12;
const SCARECROW_ID: u8 = 13;
const TREE_ID: u8 = 14;
const WATER_ID: u8 = 15;

pub struct ContentFactory {
    textures: HashMap<u8, Rc<Texture2D>>,
}

impl ContentFactory {
    pub async fn new() -> Self {
        let mut textures = HashMap::new();

        textures.insert(
            BANK_ID,
            Rc::new(
                load_texture("assets/textures/contents/bank.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            BIN_ID,
            Rc::new(
                load_texture("assets/textures/contents/bin.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            BUILDING_ID,
            Rc::new(
                load_texture("assets/textures/contents/building.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            BUSH_ID,
            Rc::new(
                load_texture("assets/textures/contents/bush.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            CHEST_ID,
            Rc::new(
                load_texture("assets/textures/contents/chest.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            COIN_ID,
            Rc::new(
                load_texture("assets/textures/contents/coin.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            FIRE_ID,
            Rc::new(
                load_texture("assets/textures/contents/fire.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            FISH_ID,
            Rc::new(
                load_texture("assets/textures/contents/fish.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            GARBAGE_ID,
            Rc::new(
                load_texture("assets/textures/contents/garbage.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            JOLLYBLOCK_ID,
            Rc::new(
                load_texture("assets/textures/contents/jollyblock.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            MARKET_ID,
            Rc::new(
                load_texture("assets/textures/contents/market.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(NONE_ID, Rc::new(Texture2D::empty()));

        textures.insert(
            ROCK_ID,
            Rc::new(
                load_texture("assets/textures/contents/rock.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            SCARECROW_ID,
            Rc::new(
                load_texture("assets/textures/contents/scarecrow.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            TREE_ID,
            Rc::new(
                load_texture("assets/textures/contents/tree.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            WATER_ID,
            Rc::new(
                load_texture("assets/textures/contents/water.png")
                    .await
                    .unwrap(),
            ),
        );

        Self { textures }
    }

    pub fn new_bank(&self, pos: Vec2) -> Bank {
        Bank {
            pos,
            offset: Vec2::new(0.0, 0.0),
            texture: self.textures[&BANK_ID].clone(),
        }
    }

    pub fn new_bin(&self, pos: Vec2) -> Bin {
        Bin {
            pos,
            offset: Vec2::new(0.0, 0.0),
            texture: self.textures[&BIN_ID].clone(),
        }
    }

    pub fn new_building(&self, pos: Vec2) -> Building {
        Building {
            pos,
            offset: Vec2::new(0.0, -128.0),
            texture: self.textures[&BUILDING_ID].clone(),
        }
    }

    pub fn new_bush(&self, pos: Vec2) -> Bush {
        Bush {
            pos,
            offset: Vec2::new(0.0, 0.0),
            texture: self.textures[&BUSH_ID].clone(),
        }
    }

    pub fn new_chest(&self, pos: Vec2) -> Chest {
        Chest {
            pos,
            offset: Vec2::new(0.0, 0.0),
            texture: self.textures[&CHEST_ID].clone(),
        }
    }

    pub fn new_coin(&self, pos: Vec2) -> Coin {
        Coin {
            pos,
            offset: Vec2::new(0.0, 0.0),
            texture: self.textures[&COIN_ID].clone(),
        }
    }

    pub fn new_fire(&self, pos: Vec2) -> Fire {
        Fire {
            pos,
            offset: Vec2::new(0.0, 0.0),
            texture: self.textures[&FIRE_ID].clone(),
            sprite: AnimatedSprite::new(
                128,
                128,
                &[Animation {
                    name: "fire_0".to_string(),
                    row: 0,
                    frames: 7,
                    fps: 12,
                }],
                true,
            ),
        }
    }

    pub fn new_fish(&self, pos: Vec2) -> Fish {
        Fish {
            pos,
            offset: Vec2::new(0.0, 0.0),
            texture: self.textures[&FISH_ID].clone(),
        }
    }

    pub fn new_garbage(&self, pos: Vec2) -> Garbage {
        Garbage {
            pos,
            offset: Vec2::new(0.0, 0.0),
            texture: self.textures[&GARBAGE_ID].clone(),
        }
    }

    pub fn new_jollyblock(&self, pos: Vec2) -> Jollyblock {
        Jollyblock {
            pos,
            offset: Vec2::new(0.0, 0.0),
            texture: self.textures[&JOLLYBLOCK_ID].clone(),
            sprite: AnimatedSprite::new(
                128,
                128,
                &[
                    Animation {
                        name: "jollyblock_0".to_string(),
                        row: 0,
                        frames: 8,
                        fps: 12,
                    },
                    Animation {
                        name: "jollyblock_1".to_string(),
                        row: 1,
                        frames: 6,
                        fps: 12,
                    },
                ],
                true,
            ),
        }
    }

    pub fn new_market(&self, pos: Vec2) -> Market {
        Market {
            pos,
            offset: Vec2::new(0.0, 0.0),
            texture: self.textures[&MARKET_ID].clone(),
            sprite: AnimatedSprite::new(
                256,
                192,
                &[Animation {
                    name: "market_0".to_string(),
                    row: 0,
                    frames: 4,
                    fps: 12,
                }],
                true,
            ),
        }
    }

    pub fn new_none(&self, _pos: Vec2) -> None {
        None {}
    }

    pub fn new_rock(&self, pos: Vec2) -> Rock {
        Rock {
            pos,
            offset: Vec2::new(0.0, 0.0),
            texture: self.textures[&ROCK_ID].clone(),
        }
    }

    pub fn new_scarecrow(&self, pos: Vec2) -> Scarecrow {
        Scarecrow {
            pos,
            offset: Vec2::new(0.0, 0.0),
            texture: self.textures[&SCARECROW_ID].clone(),
        }
    }

    pub fn new_tree(&self, pos: Vec2) -> Tree {
        Tree {
            pos,
            offset: Vec2::new(0.0, -62.0),
            texture: self.textures[&TREE_ID].clone(),
            sprite: AnimatedSprite::new(
                192,
                192,
                &[
                    Animation {
                        name: "tree_0".to_string(),
                        row: 0,
                        frames: 4,
                        fps: 6,
                    },
                    Animation {
                        name: "tree_1".to_string(),
                        row: 1,
                        frames: 2,
                        fps: 6,
                    },
                ],
                true,
            ),
        }
    }

    pub fn new_water(&self, pos: Vec2) -> Water {
        Water {
            pos,
            offset: Vec2::new(0.0, 0.0),
            texture: self.textures[&WATER_ID].clone(),
            sprite: AnimatedSprite::new(
                192,
                192,
                &[Animation {
                    name: "water_0".to_string(),
                    row: 0,
                    frames: 8,
                    fps: 12,
                }],
                true,
            ),
        }
    }

    pub fn from_rob_content(&self, pos: Vec2, content: &RobContent) -> Box<dyn Content> {
        match content {
            RobContent::Bank(_) => Box::new(self.new_bank(pos)),
            RobContent::Bin(_) => Box::new(self.new_bin(pos)),
            RobContent::Building => Box::new(self.new_building(pos)),
            RobContent::Bush(_) => Box::new(self.new_bush(pos)),
            RobContent::Coin(_) => Box::new(self.new_coin(pos)),
            RobContent::Crate(_) => Box::new(self.new_chest(pos)),
            RobContent::Fire => Box::new(self.new_fire(pos)),
            RobContent::Fish(_) => Box::new(self.new_fish(pos)),
            RobContent::Garbage(_) => Box::new(self.new_garbage(pos)),
            RobContent::JollyBlock(_) => Box::new(self.new_jollyblock(pos)),
            RobContent::Market(_) => Box::new(self.new_market(pos)),
            RobContent::Rock(_) => Box::new(self.new_rock(pos)),
            RobContent::Scarecrow => Box::new(self.new_scarecrow(pos)),
            RobContent::Tree(_) => Box::new(self.new_tree(pos)),
            RobContent::Water(_) => Box::new(self.new_water(pos)),
            RobContent::None => Box::new(self.new_none(pos)),
        }
    }
}
