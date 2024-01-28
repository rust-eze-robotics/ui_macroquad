use macroquad::prelude::*;
use macroquad::texture::Texture2D;

use crate::{
    context::Context,
    core::{is_in_window, Drawable},
};

use std::rc::Rc;

use super::{Icon, IconState};

pub struct AudioIcon {
    pub(super) pos: Vec2,
    pub(super) texture_active: Rc<Texture2D>,
    pub(super) texture_disabled: Rc<Texture2D>,
    pub(super) texture_down: Rc<Texture2D>,
    pub state: IconState,
}

impl Icon for AudioIcon {}

impl Drawable for AudioIcon {
    fn draw(&mut self, context: &Context) {
        let texture = match self.state {
            IconState::Active => &self.texture_active,
            IconState::Disabled => &self.texture_disabled,
            IconState::Down => &self.texture_down,
        };

        draw_texture(&texture, self.pos.x, self.pos.y, LIGHTGRAY);
    }
}
