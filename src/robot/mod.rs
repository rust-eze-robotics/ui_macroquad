use std::{cell::RefCell, rc::Rc, thread, time::Duration};

use macroquad::experimental::coroutines::wait_seconds;
use robotics_lib::{
    energy::Energy,
    event::events::Event,
    interface::{go, one_direction_view, robot_map, robot_view},
    runner::{backpack::BackPack, Robot, Runnable},
    world::{coordinates::Coordinate, World as RobWorld},
};

use crate::{core::Drawable, world::World};

pub struct MyRobot {
    pub robot: Robot,
    pub world: Rc<RefCell<World>>,
}

impl Runnable for MyRobot {
    fn process_tick(&mut self, world: &mut RobWorld) {
        if let Some(map) = robot_map(world) {
            go(self, world, robotics_lib::interface::Direction::Down);
            go(self, world, robotics_lib::interface::Direction::Right);
            robot_view(self, world);
            self.world.borrow_mut().update(&map);
        }
    }

    fn handle_event(&mut self, event: Event) {}

    fn get_energy(&self) -> &Energy {
        &self.robot.energy
    }
    fn get_energy_mut(&mut self) -> &mut Energy {
        &mut self.robot.energy
    }

    fn get_coordinate(&self) -> &Coordinate {
        &self.robot.coordinate
    }
    fn get_coordinate_mut(&mut self) -> &mut Coordinate {
        &mut self.robot.coordinate
    }

    fn get_backpack(&self) -> &BackPack {
        &self.robot.backpack
    }
    fn get_backpack_mut(&mut self) -> &mut BackPack {
        &mut self.robot.backpack
    }
}
