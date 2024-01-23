use crate::core::Drawable;

pub mod bank;
pub mod bin;
pub mod building;
pub mod bush;
pub mod chest;
pub mod coin;
pub mod fire;
pub mod fish;
pub mod garbage;
pub mod jollyblock;
pub mod market;
pub mod none;
pub mod rock;
pub mod scarecrow;
pub mod tree;
pub mod water;

pub trait Content: Drawable {}
