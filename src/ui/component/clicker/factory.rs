

use macroquad::prelude::*;

use crate::ui::component::icon::factory::IconFactory;

use super::{Clicker};

pub struct ClickerFactory {}

impl ClickerFactory {
    pub async fn new() -> Self {
        Self {}
    }

    pub fn new_plus_clicker(&self, icon_factory: &IconFactory) -> Clicker {
        Clicker {
            pos: Vec2::default(),
            size: Vec2::new(64.0, 64.0),
            icon: icon_factory.new_plus_icon(Vec2::new(64.0, 64.0)),
        }
    }

    pub fn new_minus_clicker(&self, icon_factory: &IconFactory) -> Clicker {
        Clicker {
            pos: Vec2::default(),
            size: Vec2::new(64.0, 64.0),
            icon: icon_factory.new_minus_icon(Vec2::new(64.0, 64.0)),
        }
    }
}
