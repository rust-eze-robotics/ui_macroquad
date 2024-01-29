use std::{cell::RefCell, rc::Rc};

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

pub trait UiComponent: Drawable {
    fn update(&mut self);
    fn handle(&mut self);
}

pub struct Ui {
    pub map: Map,
    pub components: Vec<Box<dyn UiComponent>>,
}

impl Ui {
    pub fn update(&mut self) {
        for component in self.components.iter_mut() {
            component.update();
        }
    }

    pub fn handle(&mut self) {
        for component in self.components.iter_mut() {
            component.handle();
        }
    }
}

impl Ui {
    pub async fn new(world: Rc<RefCell<World>>) -> Self {
        let icon_factory = IconFactory::new().await;
        let button_factory = ButtonFactory::new().await;

        let mut components = Vec::<Box<dyn UiComponent>>::new();
        components.push(Box::new(button_factory.new_audio_button(&icon_factory)));

        Self {
            map: Map::new(world),
            components,
        }
    }
}

impl Drawable for Ui {
    fn draw(&mut self, context: &crate::context::Context) {
        for component in self.components.iter_mut() {
            component.draw(context);
        }
    }
}
