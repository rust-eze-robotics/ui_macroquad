use macroquad::prelude::*;
use macroquad::texture::Texture2D;

use crate::{
    context::Context,
    core::{is_in_window, Drawable},
};

use std::rc::Rc;

pub mod factory;

#[derive(Debug, PartialEq)]
pub enum IconState {
    Active,
    Disabled,
    Down,
}

pub struct Icon {
    pub(super) pos: Vec2,
    pub(super) texture_active: Rc<Texture2D>,
    pub(super) texture_disabled: Rc<Texture2D>,
    pub(super) texture_down: Rc<Texture2D>,
    pub state: IconState,
}

impl Drawable for Icon {
    fn draw(&mut self, context: &Context) {
        let texture = match self.state {
            IconState::Active => &self.texture_active,
            IconState::Disabled => &self.texture_disabled,
            IconState::Down => &self.texture_down,
        };

        draw_texture(&texture, self.pos.x, self.pos.y, LIGHTGRAY);
    }
}
