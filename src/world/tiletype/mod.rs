use std::{collections::HashMap, rc::Rc};

use macroquad::{
    math::Vec2,
    texture::{load_texture, Texture2D},
};

use crate::core::Drawable;

use self::{
    deep_water::DeepWater, grass::Grass, hill::Hill, lava::Lava, mountain::Mountain, sand::Sand,
    shallow_water::ShallowWater, snow::Snow, street::Street, teleport::Teleport, wall::Wall,
};

pub mod deep_water;
pub mod grass;
pub mod hill;
pub mod lava;
pub mod mountain;
pub mod sand;
pub mod shallow_water;
pub mod snow;
pub mod street;
pub mod teleport;
pub mod wall;

pub trait Tiletype: Drawable {}

const DEEP_WATER_ID: usize = 0;
const GRASS_ID: usize = 1;
const HILL_ID: usize = 2;
const LAVA_ID: usize = 3;
const MOUNTAIN_ID: usize = 4;
const SAND_ID: usize = 5;
const SHALLOW_WATER_ID: usize = 6;
const SNOW_ID: usize = 7;
const STREET_ID: usize = 8;
const TELEPORT_ID: usize = 9;
const WALL_ID: usize = 10;

pub struct TiletypeFactory {
    textures: HashMap<usize, Rc<Texture2D>>,
}

impl TiletypeFactory {
    pub async fn new() -> Self {
        let mut textures = HashMap::new();

        textures.insert(
            DEEP_WATER_ID,
            Rc::new(
                load_texture("data/assets/tiletypes/deep_water/deep_water.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            GRASS_ID,
            Rc::new(
                load_texture("data/assets/tiletypes/grass/grass.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            HILL_ID,
            Rc::new(
                load_texture("data/assets/tiletypes/hill/hill.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            LAVA_ID,
            Rc::new(
                load_texture("data/assets/tiletypes/lava/lava.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            MOUNTAIN_ID,
            Rc::new(
                load_texture("data/assets/tiletypes/mountain/mountain.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            SAND_ID,
            Rc::new(
                load_texture("data/assets/tiletypes/sand/sand.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            SHALLOW_WATER_ID,
            Rc::new(
                load_texture("data/assets/tiletypes/shallow_water/shallow_water.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            SNOW_ID,
            Rc::new(
                load_texture("data/assets/tiletypes/snow/snow.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            STREET_ID,
            Rc::new(
                load_texture("data/assets/tiletypes/street/street.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            TELEPORT_ID,
            Rc::new(
                load_texture("data/assets/tiletypes/grass/grass.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            WALL_ID,
            Rc::new(
                load_texture("data/assets/tiletypes/wall/wall.png")
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
}
