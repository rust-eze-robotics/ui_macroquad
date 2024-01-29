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
pub mod factory;
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

pub trait TileType: Drawable {}
