use macroquad::prelude::*;

use crate::{
    core::{context::Context, is_down, Drawable},
    ui::{
        component::icon::{Icon, IconState},
        UiItem,
    },
};

pub mod factory;

pub struct Clicker {
    pub pos: Vec2,
    pub size: Vec2,
    pub icon: Icon,
}

impl UiItem for Clicker {
    fn update_gui(&mut self, _context: &Context) {
        self.icon.pos = self.pos;
    }

    fn handle_input(&mut self, _context: &Context) {
        if is_down(&self.pos, &self.size) {
            self.icon.state = IconState::Down;
        } else {
            self.icon.state = IconState::Active;
        }
    }
}

impl Drawable for Clicker {
    fn draw(&mut self, context: &Context) {
        self.icon.draw(context);
    }
}
