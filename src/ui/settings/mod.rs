use macroquad::prelude::*;

use crate::core::{context::Context, Drawable};

use super::{
    component::{
        clicker::factory::ClickerFactory,
        icon::{factory::IconFactory, Icon},
        stepper::{self, factory::StepperFactory, Stepper},
    },
    UiItem,
};

pub const SETTINGS_MODAL_SIZE: Vec2 = Vec2::new(400.0, 250.0);

pub struct SettingsModal {
    pub pos: Vec2,
    pub volume_stepper: Stepper,
    pub speed_stepper: Stepper,
}

impl SettingsModal {
    pub async fn new(
        icon_factory: &IconFactory,
        clicker_factory: &ClickerFactory,
        stepper_factory: &StepperFactory,
    ) -> SettingsModal {
        let pos = Vec2::new(
            (screen_width() - SETTINGS_MODAL_SIZE.x) / 2.0,
            (screen_height() - SETTINGS_MODAL_SIZE.y) / 2.0,
        );

        SettingsModal {
            pos,
            volume_stepper: stepper_factory.new_volume_stepper(
                icon_factory,
                clicker_factory,
                pos + Vec2::new(20.0, 20.0),
            ),
            speed_stepper: stepper_factory.new_speed_stepper(
                icon_factory,
                clicker_factory,
                pos + Vec2::new(20.0, 50.0),
            ),
        }
    }
}

impl UiItem for SettingsModal {
    fn update_gui(&mut self, context: &Context) {
        self.pos = Vec2::new(
            (screen_width() - SETTINGS_MODAL_SIZE.x) / 2.0,
            (screen_height() - SETTINGS_MODAL_SIZE.y) / 2.0,
        );

        self.volume_stepper.pos = self.pos + Vec2::new(20.0, 20.0);
        self.speed_stepper.pos = self.pos + Vec2::new(20.0, 70.0);

        self.volume_stepper.update_gui(context);
        self.speed_stepper.update_gui(context);
    }

    fn handle_input(&mut self, context: &Context) {
        self.volume_stepper.handle_input(context);
        self.speed_stepper.handle_input(context);
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

            self.volume_stepper.draw(context);
            self.speed_stepper.draw(context);
        }
    }
}
