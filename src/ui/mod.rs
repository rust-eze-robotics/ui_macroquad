use std::{cell::RefCell, rc::Rc};

use crate::{core::Drawable, world::World};

use self::map::Map;

pub mod map;

pub struct Ui {
    pub map: Map,
}

impl Ui {
    pub fn new(world: Rc<RefCell<World>>) -> Self {
        Self {
            map: Map::new(world),
        }
    }
}

impl Drawable for Ui {
    fn draw(&mut self, context: &crate::context::Context) {
        self.map.draw(context);
    }
}
