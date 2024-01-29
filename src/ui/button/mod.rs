use macroquad::{
    math::Vec2,
    window::{screen_height, screen_width},
};

use crate::core::Drawable;

pub mod factory;
pub mod square;

#[derive(Debug, PartialEq)]
pub enum ButtonState {
    Active,
    Down,
    Disabled,
    Hovered,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AnchorPosition {
    TOP_LEFT(Vec2),
    TOP_RIGHT(Vec2),
    DOWN_LEFT(Vec2),
    DOWN_RIGHT(Vec2),
}

pub fn get_current_anchor_position(anchor_pos: AnchorPosition) -> Vec2 {
    match anchor_pos {
        AnchorPosition::TOP_LEFT(v) => Vec2::new(0.0, 0.0) + v,
        AnchorPosition::TOP_RIGHT(v) => Vec2::new(screen_width(), 0.0) + v,
        AnchorPosition::DOWN_LEFT(v) => Vec2::new(0.0, screen_height()) + v,
        AnchorPosition::DOWN_RIGHT(v) => Vec2::new(screen_width(), screen_height()) + v,
    }
}
