use std::{cell::RefCell, rc::Rc};

use crate::{context::Context, core::Drawable, world::World};

pub struct Map {
    world: Rc<RefCell<World>>,
}

impl Map {
    pub fn new(world: Rc<RefCell<World>>) -> Self {
        Self { world }
    }
}

impl Drawable for Map {
    fn draw(&mut self, context: &Context) {
        self.world.borrow_mut().draw(context);
    }
}
