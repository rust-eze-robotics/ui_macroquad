use macroquad::prelude::*;

use crate::{
    core::{SETTINGS_STEPPER_SIZE, TICK_DURATION_DEFAULT, TICK_DURATION_MAX, TICK_DURATION_MIN},
    ui::component::{clicker::factory::ClickerFactory, icon::factory::IconFactory},
};

use super::Stepper;

pub struct StepperFactory {}

impl StepperFactory {
    pub async fn new() -> Self {
        Self {}
    }

    pub fn new_volume_stepper(
        &self,
        icon_factory: &IconFactory,
        clicker_factory: &ClickerFactory,
        pos: Vec2,
    ) -> Stepper {
        Stepper {
            pos,
            size: SETTINGS_STEPPER_SIZE,
            title: String::from("Volume"),
            value: 50.0,
            min_value: 10.0,
            max_value: 100.0,
            increment: 10.0,
            minus_clicker: clicker_factory.new_minus_clicker(icon_factory),
            plus_clicker: clicker_factory.new_plus_clicker(icon_factory),
        }
    }

    pub fn new_tick_stepper(
        &self,
        icon_factory: &IconFactory,
        clicker_factory: &ClickerFactory,
        pos: Vec2,
    ) -> Stepper {
        Stepper {
            pos,
            size: SETTINGS_STEPPER_SIZE,
            title: String::from("Tick"),
            value: TICK_DURATION_DEFAULT.as_millis() as f32,
            min_value: TICK_DURATION_MIN.as_millis() as f32,
            max_value: TICK_DURATION_MAX.as_millis() as f32,
            increment: 500.0,
            minus_clicker: clicker_factory.new_minus_clicker(icon_factory),
            plus_clicker: clicker_factory.new_plus_clicker(icon_factory),
        }
    }
}
