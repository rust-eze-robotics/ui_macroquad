use macroquad::prelude::*;

use crate::core::{context::Context, Drawable};

use super::{
    component::icon::{factory::IconFactory, Icon},
    UiComponent,
};

const SETTINGS_MODAL_SIZE: Vec2 = Vec2::new(384.0, 384.0);

pub struct SettingsModal {
    pub pos: Vec2,
}

impl SettingsModal {
    pub async fn new(icon_factory: &IconFactory) -> SettingsModal {
        SettingsModal {
            pos: Vec2::new(
                (screen_width() - SETTINGS_MODAL_SIZE.x) / 2.0,
                (screen_height() - SETTINGS_MODAL_SIZE.y) / 2.0,
            ),
        }
    }
}

impl UiComponent for SettingsModal {
    fn update_gui(&mut self, context: &Context) {
        self.pos = Vec2::new(
            (screen_width() - SETTINGS_MODAL_SIZE.x) / 2.0,
            (screen_height() - SETTINGS_MODAL_SIZE.y) / 2.0,
        );
    }

    fn handle_input(&mut self, context: &Context) {
        // if is_down(&self.pos, &self.size) {
        //     self.state = ButtonState::Down;
        //     self.icon.state = IconState::Pressed;
        // } else if is_released(&self.pos, &self.size) {
        //     self.on = !self.on;

        //     if self.on {
        //         self.state = ButtonState::Active;
        //         self.icon.state = IconState::Active;
        //     } else {
        //         self.state = ButtonState::Disabled;
        //         self.icon.state = IconState::Disabled;
        //     }
        // } else if is_hovered(&self.pos, &self.size) {
        //     if self.on {
        //         self.state = ButtonState::Hovered;
        //     }
        // } else {
        //     if self.on {
        //         self.state = ButtonState::Active;
        //         self.icon.state = IconState::Active;
        //     } else {
        //         self.state = ButtonState::Disabled;
        //         self.icon.state = IconState::Disabled;
        //     }
        // }
    }
}

impl Drawable for SettingsModal {
    fn draw(&mut self, context: &Context) {
        if context.settings_open {
            draw_rectangle(
                self.pos.x,
                self.pos.y,
                SETTINGS_MODAL_SIZE.x,
                SETTINGS_MODAL_SIZE.y,
                BEIGE,
            );
            draw_rectangle_lines(
                self.pos.x,
                self.pos.y,
                SETTINGS_MODAL_SIZE.x,
                SETTINGS_MODAL_SIZE.y,
                5.0,
                GOLD,
            );
        }
    }
}
