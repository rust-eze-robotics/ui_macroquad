use std::{cell::RefCell, rc::Rc};

use robotics_lib::{
    event::events::Event,
    interface::{robot_map},
    world::{World as RobWorld},
};

use ui_lib::RunnableUi;

use crate::{core::events::EventsHandler, robot::Robot, ui::Ui, world::World};

pub struct Wrapper {
    pub robot: Rc<RefCell<Robot>>,
    pub world: Rc<RefCell<World>>,
    pub ui: Rc<RefCell<Ui>>,
    pub events_handler: Rc<RefCell<EventsHandler>>,
}

impl Wrapper {
    pub fn new(
        robot: Rc<RefCell<Robot>>,
        world: Rc<RefCell<World>>,
        ui: Rc<RefCell<Ui>>,
        events_handler: Rc<RefCell<EventsHandler>>,
    ) -> Self {
        Self {
            robot,
            world,
            ui,
            events_handler,
        }
    }
}

impl RunnableUi for Wrapper {
    fn process_tick(&mut self, world: &mut RobWorld) {
        let map = robot_map(world).unwrap();
        self.world.borrow_mut().update_visibility(&map);
    }

    fn handle_event(&mut self, event: Event) {
        self.events_handler.borrow_mut().push(event);
    }
}
