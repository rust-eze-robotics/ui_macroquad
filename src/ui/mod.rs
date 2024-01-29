use std::{cell::RefCell, rc::Rc, time::Instant};

use macroquad::{
    math::Vec2,
    window::{screen_height, screen_width},
};

use crate::{core::Drawable, world::World};

use self::{
    button::{factory::ButtonFactory, square::SquareButton},
    icon::factory::IconFactory,
    map::Map,
};

pub mod button;
pub mod icon;
pub mod map;

pub struct Ui {
    pub map: Map,
    pub audio_button: SquareButton,
}

impl Ui {
    pub fn update(&mut self) {
        self.audio_button.update();
    }
}

impl Ui {
    pub async fn new(world: Rc<RefCell<World>>) -> Self {
        let icon_factory = IconFactory::new().await;
        let button_factory = ButtonFactory::new().await;

        Self {
            map: Map::new(world),
            audio_button: button_factory.new_audio_button(
                &icon_factory,
                Vec2::new(screen_width() - 64.0, screen_height() - 64.0),
            ),
        }
    }
}

impl Drawable for Ui {
    fn draw(&mut self, context: &crate::context::Context) {
        self.audio_button.draw(context);
    }
}
