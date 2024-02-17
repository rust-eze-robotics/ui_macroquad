use macroquad::prelude::*;
use macroquad::texture::Texture2D;

use crate::{
    core::{
        context::Context, get_current_anchor_position, is_down, is_hovered, is_released,
        AnchorPosition, Drawable,
    },
    ui::{
        component::icon::{Icon, IconState},
        UiItem,
    },
};

use std::rc::Rc;

pub mod factory;

#[derive(Debug, PartialEq)]
pub enum ButtonState {
    Active,
    Down,
    Disabled,
    Hovered,
}

pub struct Button {
    pub pos: Vec2,
    pub anchor_pos: AnchorPosition,
    pub size: Vec2,
    pub icon: Icon,
    pub state: ButtonState,
    pub on: bool,
    pub texture_active: Rc<Texture2D>,
    pub texture_down: Rc<Texture2D>,
    pub texture_disabled: Rc<Texture2D>,
    pub texture_hovered: Rc<Texture2D>,
}

impl UiItem for Button {
    fn update_gui(&mut self) {
        self.pos = get_current_anchor_position(self.anchor_pos);
        self.icon.pos = self.pos;
    }

    fn handle_input(&mut self) {
        if is_down(&self.pos, &self.size) {
            self.state = ButtonState::Down;
            self.icon.state = IconState::Down;
        } else if is_released(&self.pos, &self.size) {
            self.on = !self.on;

            if self.on {
                self.state = ButtonState::Active;
                self.icon.state = IconState::Active;
            } else {
                self.state = ButtonState::Disabled;
                self.icon.state = IconState::Disabled;
            }
        } else if is_hovered(&self.pos, &self.size) {
            if self.on {
                self.state = ButtonState::Hovered;
            }
        } else {
            if self.on {
                self.state = ButtonState::Active;
                self.icon.state = IconState::Active;
            } else {
                self.state = ButtonState::Disabled;
                self.icon.state = IconState::Disabled;
            }
        }
    }
}

impl Drawable for Button {
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
