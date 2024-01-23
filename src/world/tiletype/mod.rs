use crate::core::Drawable;

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
