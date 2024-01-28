use macroquad::prelude::*;
use macroquad::texture::Texture2D;

use crate::{
    context::Context,
    core::{is_down, is_hovered, is_in_window, is_released, Drawable},
    ui::icon::{audio::AudioIcon, IconState},
};

use std::{rc::Rc, time::Instant};

use super::{Button, ButtonState};

pub struct AudioButton {
    pub(super) pos: Vec2,
    pub(super) size: Vec2,
    pub(super) texture_active: Rc<Texture2D>,
    pub(super) texture_down: Rc<Texture2D>,
    pub(super) texture_disabled: Rc<Texture2D>,
    pub(super) texture_hovered: Rc<Texture2D>,
    pub(super) icon: AudioIcon,
    pub(super) state: ButtonState,
    pub(super) active: bool,
}

impl AudioButton {
    pub fn update(&mut self) {
        if is_down(&self.pos, &self.size) {
            self.state = ButtonState::Down;
            self.icon.state = IconState::Down;
        } else if is_released(&self.pos, &self.size) {
            self.active = !self.active;

            if self.active {
                self.state = ButtonState::Active;
                self.icon.state = IconState::Active;
            } else {
                self.state = ButtonState::Disabled;
                self.icon.state = IconState::Disabled;
            }
        } else if is_hovered(&self.pos, &self.size) {
            if self.active {
                self.state = ButtonState::Hovered;
            }
        } else {
            if self.active {
                self.state = ButtonState::Active;
                self.icon.state = IconState::Active;
            } else {
                self.state = ButtonState::Disabled;
                self.icon.state = IconState::Disabled;
            }
        }
    }
}

impl Button for AudioButton {}

impl Drawable for AudioButton {
    fn draw(&mut self, context: &Context) {
        let texture = match self.state {
            ButtonState::Active => &self.texture_active,
            ButtonState::Down => &self.texture_down,
            ButtonState::Disabled => &self.texture_disabled,
            ButtonState::Hovered => &self.texture_hovered,
        };

        draw_texture(&texture, self.pos.x, self.pos.y, LIGHTGRAY);

        self.icon.draw(context);
    }
}
