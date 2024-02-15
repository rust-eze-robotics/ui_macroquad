use macroquad::prelude::*;
use macroquad::texture::Texture2D;

use crate::{core::context::Context, core::Drawable};

use std::rc::Rc;

pub mod factory;

#[derive(Debug, PartialEq)]
pub enum IconState {
    Active,
    Disabled,
    Down,
}

pub struct Icon {
    pub pos: Vec2,
    pub texture_active: Rc<Texture2D>,
    pub texture_disabled: Rc<Texture2D>,
    pub texture_down: Rc<Texture2D>,
    pub state: IconState,
}

impl Drawable for Icon {
    fn draw(&mut self, _context: &Context) {
        let texture = match self.state {
            IconState::Active => &self.texture_active,
            IconState::Disabled => &self.texture_disabled,
            IconState::Down => &self.texture_down,
        };

        draw_texture(&texture, self.pos.x, self.pos.y, LIGHTGRAY);
    }
}
