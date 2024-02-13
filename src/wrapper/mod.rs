use std::{cell::RefCell, rc::Rc};

use robotics_lib::{
    energy::Energy,
    event::events::Event,
    interface::{robot_map, robot_view},
    runner::{backpack::BackPack, Robot as RobRobot, Runnable},
    world::{coordinates::Coordinate, World as RobWorld},
};
use rust_eze_spotlight::Spotlight;
use rust_eze_tomtom::TomTom;

use crate::{core::events::EventsHandler, robot::Robot, world::World};

pub struct Wrapper {
    pub rob_robot: RobRobot,
    pub robot: Rc<RefCell<Robot>>,
    pub world: Rc<RefCell<World>>,
    pub events_handler: Rc<RefCell<EventsHandler>>,
}

impl Wrapper {
    pub fn new(
        rob_robot: RobRobot,
        robot: Rc<RefCell<Robot>>,
        world: Rc<RefCell<World>>,
        events_handler: Rc<RefCell<EventsHandler>>,
    ) -> Self {
        Self {
            rob_robot,
            robot,
            world,
            events_handler,
        }
    }
}

impl Runnable for Wrapper {
    fn process_tick(&mut self, world: &mut RobWorld) {
        //

        Spotlight::illuminate(self, world, 10);
        TomTom::go_to_tile(
            self,
            world,
            false,
            None,
            Some(rust_eze_tomtom::plain::PlainContent::Bush),
        );
        robot_view(self, world);

        //

        let map = robot_map(world).unwrap();
        self.world.borrow_mut().update_visibility(&map);
    }

    fn handle_event(&mut self, event: Event) {
        self.events_handler.borrow_mut().push(event);
    }

    fn get_energy(&self) -> &Energy {
        &self.rob_robot.energy
    }
    fn get_energy_mut(&mut self) -> &mut Energy {
        &mut self.rob_robot.energy
    }

    fn get_coordinate(&self) -> &Coordinate {
        &self.rob_robot.coordinate
    }
    fn get_coordinate_mut(&mut self) -> &mut Coordinate {
        &mut self.rob_robot.coordinate
    }

    fn get_backpack(&self) -> &BackPack {
        &self.rob_robot.backpack
    }
    fn get_backpack_mut(&mut self) -> &mut BackPack {
        &mut self.rob_robot.backpack
    }
}
