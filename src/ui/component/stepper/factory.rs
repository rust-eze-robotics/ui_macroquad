

use macroquad::prelude::*;

use crate::ui::component::{clicker::factory::ClickerFactory, icon::factory::IconFactory};

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
            title: String::from("Volume"),
            value: 50.0,
            min_value: 0.0,
            max_value: 100.0,
            increment: 10.0,
            minus_clicker: clicker_factory.new_minus_clicker(icon_factory),
            plus_clicker: clicker_factory.new_plus_clicker(icon_factory),
        }
    }

    pub fn new_speed_stepper(
        &self,
        icon_factory: &IconFactory,
        clicker_factory: &ClickerFactory,
        pos: Vec2,
    ) -> Stepper {
        Stepper {
            pos,
            title: String::from("Speed"),
            value: 50.0,
            min_value: 0.0,
            max_value: 100.0,
            increment: 10.0,
            minus_clicker: clicker_factory.new_minus_clicker(icon_factory),
            plus_clicker: clicker_factory.new_plus_clicker(icon_factory),
        }
    }
}
