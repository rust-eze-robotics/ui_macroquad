use macroquad::{
    math::Vec2,
    window::{screen_height, screen_width},
};

use crate::core::Drawable;

pub mod factory;
pub mod square_button;

#[derive(Debug, PartialEq)]
pub enum ButtonState {
    Active,
    Down,
    Disabled,
    Hovered,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AnchorPosition {
    TopLeft(Vec2),
    TopRight(Vec2),
    DownLeft(Vec2),
    DownRight(Vec2),
}

pub fn get_current_anchor_position(anchor_pos: AnchorPosition) -> Vec2 {
    match anchor_pos {
        AnchorPosition::TopLeft(v) => Vec2::new(0.0, 0.0) + v,
        AnchorPosition::TopRight(v) => Vec2::new(screen_width(), 0.0) + v,
        AnchorPosition::DownLeft(v) => Vec2::new(0.0, screen_height()) + v,
        AnchorPosition::DownRight(v) => Vec2::new(screen_width(), screen_height()) + v,
    }
}
