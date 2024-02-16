use std::{collections::HashMap, rc::Rc};

use macroquad::prelude::*;
use robotics_lib::world::tile::TileType as RobTileType;

use super::{
    deep_water::DeepWater, grass::Grass, hill::Hill, lava::Lava, mountain::Mountain, sand::Sand,
    shallow_water::ShallowWater, snow::Snow, street::Street, teleport::Teleport, wall::Wall,
    TileType,
};

const DEEP_WATER_ID: u8 = 0;
const GRASS_ID: u8 = 1;
const HILL_ID: u8 = 2;
const LAVA_ID: u8 = 3;
const MOUNTAIN_ID: u8 = 4;
const SAND_ID: u8 = 5;
const SHALLOW_WATER_ID: u8 = 6;
const SNOW_ID: u8 = 7;
const STREET_ID: u8 = 8;
const TELEPORT_ID: u8 = 9;
const WALL_ID: u8 = 10;

pub struct TileTypeFactory {
    textures: HashMap<u8, Rc<Texture2D>>,
}

impl TileTypeFactory {
    pub async fn new() -> Self {
        let mut textures = HashMap::new();

        textures.insert(
            DEEP_WATER_ID,
            Rc::new(
                load_texture("assets/textures/tiletypes/deep_water.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            GRASS_ID,
            Rc::new(
                load_texture("assets/textures/tiletypes/grass.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            HILL_ID,
            Rc::new(
                load_texture("assets/textures/tiletypes/hill.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            LAVA_ID,
            Rc::new(
                load_texture("assets/textures/tiletypes/lava.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            MOUNTAIN_ID,
            Rc::new(
                load_texture("assets/textures/tiletypes/mountain.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            SAND_ID,
            Rc::new(
                load_texture("assets/textures/tiletypes/sand.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            SHALLOW_WATER_ID,
            Rc::new(
                load_texture("assets/textures/tiletypes/shallow_water.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            SNOW_ID,
            Rc::new(
                load_texture("assets/textures/tiletypes/snow.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            STREET_ID,
            Rc::new(
                load_texture("assets/textures/tiletypes/street.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            TELEPORT_ID,
            Rc::new(
                load_texture("assets/textures/tiletypes/grass.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            WALL_ID,
            Rc::new(
                load_texture("assets/textures/tiletypes/wall.png")
                    .await
                    .unwrap(),
            ),
        );

        Self { textures }
    }

    pub fn new_deep_water(&self, pos: Vec2) -> DeepWater {
        DeepWater {
            pos,
            offset: Vec2::new(0.0, 0.0),
            texture: self.textures[&DEEP_WATER_ID].clone(),
        }
    }

    pub fn new_grass(&self, pos: Vec2) -> Grass {
        Grass {
            pos,
            offset: Vec2::new(0.0, 0.0),
            texture: self.textures[&GRASS_ID].clone(),
        }
    }

    pub fn new_hill(&self, pos: Vec2) -> Hill {
        Hill {
            pos,
            offset: Vec2::new(0.0, 0.0),
            texture: self.textures[&HILL_ID].clone(),
        }
    }

    pub fn new_lava(&self, pos: Vec2) -> Lava {
        Lava {
            pos,
            offset: Vec2::new(0.0, 0.0),
            texture: self.textures[&LAVA_ID].clone(),
        }
    }

    pub fn new_mountain(&self, pos: Vec2) -> Mountain {
        Mountain {
            pos,
            offset: Vec2::new(0.0, 0.0),
            texture: self.textures[&MOUNTAIN_ID].clone(),
        }
    }

    pub fn new_sand(&self, pos: Vec2) -> Sand {
        Sand {
            pos,
            offset: Vec2::new(0.0, 0.0),
            texture: self.textures[&SAND_ID].clone(),
        }
    }

    pub fn new_shallow_water(&self, pos: Vec2) -> ShallowWater {
        ShallowWater {
            pos,
            offset: Vec2::new(0.0, 0.0),
            texture: self.textures[&SHALLOW_WATER_ID].clone(),
        }
    }

    pub fn new_snow(&self, pos: Vec2) -> Snow {
        Snow {
            pos,
            offset: Vec2::new(0.0, 0.0),
            texture: self.textures[&SNOW_ID].clone(),
        }
    }

    pub fn new_street(&self, pos: Vec2) -> Street {
        Street {
            pos,
            offset: Vec2::new(0.0, 0.0),
            texture: self.textures[&STREET_ID].clone(),
        }
    }

    pub fn new_teleport(&self, pos: Vec2) -> Teleport {
        Teleport {
            pos,
            offset: Vec2::new(0.0, 0.0),
            texture: self.textures[&TELEPORT_ID].clone(),
        }
    }

    pub fn new_wall(&self, pos: Vec2) -> Wall {
        Wall {
            pos,
            offset: Vec2::new(0.0, 0.0),
            texture: self.textures[&WALL_ID].clone(),
        }
    }

    pub fn from_rob_tile_type(&self, pos: Vec2, tile_type: &RobTileType) -> Box<dyn TileType> {
        match tile_type {
            RobTileType::DeepWater => Box::new(self.new_deep_water(pos)),
            RobTileType::Grass => Box::new(self.new_grass(pos)),
            RobTileType::Hill => Box::new(self.new_hill(pos)),
            RobTileType::Lava => Box::new(self.new_lava(pos)),
            RobTileType::Sand => Box::new(self.new_mountain(pos)),
            RobTileType::Mountain => Box::new(self.new_sand(pos)),
            RobTileType::ShallowWater => Box::new(self.new_shallow_water(pos)),
            RobTileType::Snow => Box::new(self.new_snow(pos)),
            RobTileType::Street => Box::new(self.new_street(pos)),
            RobTileType::Teleport(_) => Box::new(self.new_teleport(pos)),
            RobTileType::Wall => Box::new(self.new_wall(pos)),
        }
    }
}
