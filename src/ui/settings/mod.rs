use macroquad::prelude::*;

use crate::core::{context::Context, Drawable, SETTINGS_MODAL_SIZE};

use super::{
    component::{
        clicker::factory::ClickerFactory,
        icon::factory::IconFactory,
        stepper::{factory::StepperFactory, Stepper},
    },
    UiItem,
};

pub struct SettingsModal {
    pub pos: Vec2,
    volume_stepper: Stepper,
    tick_stepper: Stepper,
}

impl SettingsModal {
    pub async fn new(
        icon_factory: &IconFactory,
        clicker_factory: &ClickerFactory,
        stepper_factory: &StepperFactory,
    ) -> Self {
        let pos = Vec2::new(
            (screen_width() - SETTINGS_MODAL_SIZE.x) / 2.0,
            (screen_height() - SETTINGS_MODAL_SIZE.y) / 2.0,
        );

        Self {
            pos,
            volume_stepper: stepper_factory.new_volume_stepper(
                icon_factory,
                clicker_factory,
                pos + Vec2::new(20.0, 20.0),
            ),
            tick_stepper: stepper_factory.new_tick_stepper(
                icon_factory,
                clicker_factory,
                pos + Vec2::new(20.0, 50.0),
            ),
        }
    }

    pub fn get_volume(&self) -> f32 {
        self.volume_stepper.value
    }

    pub fn get_speed(&self) -> f32 {
        self.tick_stepper.value
    }
}

impl UiItem for SettingsModal {
    fn update_gui(&mut self, context: &Context) {
        self.pos = Vec2::new(
            (screen_width() - SETTINGS_MODAL_SIZE.x) / 2.0,
            (screen_height() - SETTINGS_MODAL_SIZE.y) / 2.0,
        );

        self.volume_stepper.pos.x =
            self.pos.x + (SETTINGS_MODAL_SIZE.x - self.volume_stepper.size.x) / 2.0;
        self.volume_stepper.pos.y = self.pos.y + (SETTINGS_MODAL_SIZE.y * 0.25);
        self.tick_stepper.pos.x =
            self.pos.x + (SETTINGS_MODAL_SIZE.x - self.tick_stepper.size.x) / 2.0;
        self.tick_stepper.pos.y = self.pos.y + (SETTINGS_MODAL_SIZE.y * 0.55);

        self.volume_stepper.update_gui(context);
        self.tick_stepper.update_gui(context);
    }

    fn handle_input(&mut self, context: &Context) {
        self.volume_stepper.handle_input(context);
        self.tick_stepper.handle_input(context);
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
                Color::from_rgba(190, 162, 136, 255),
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
            self.tick_stepper.draw(context);
        }
    }
}
