use std::{cell::RefCell, rc::Rc};

use robotics_lib::{
    energy::{self, Energy},
    event::events::Event,
    interface::{go, look_at_sky, one_direction_view, robot_map, robot_view, teleport, Direction},
    runner::{backpack::BackPack, Robot as RobRobot, Runnable},
    world::{coordinates::Coordinate, World as RobWorld},
};
use rust_eze_spotlight::Spotlight;
use rust_eze_tomtom::{
    path::{Action, Path},
    TomTom,
};

use crate::world::World;

use self::robot::Robot;

pub mod robot;

pub struct Ai {
    pub rob_robot: RobRobot,
    pub robot: Rc<RefCell<Robot>>,
    pub world: Rc<RefCell<World>>,
}

impl Ai {
    pub fn new(rob_robot: RobRobot, robot: Rc<RefCell<Robot>>, world: Rc<RefCell<World>>) -> Self {
        Self {
            rob_robot,
            robot,
            world,
        }
    }
}

impl Runnable for Ai {
    fn process_tick(&mut self, world: &mut RobWorld) {
        //

        go(self, world, Direction::Right);
        go(self, world, Direction::Down);
        robot_view(self, world);

        //

        self.robot.borrow_mut().update_pos(
            self.get_coordinate().get_row(),
            self.get_coordinate().get_col(),
        );

        let map = robot_map(world).unwrap();
        self.world.borrow_mut().update_visibility(&map);
    }

    fn handle_event(&mut self, event: Event) {
        match event {
            Event::TileContentUpdated(tile, (row, col)) => {
                self.world.borrow_mut().update_tile(tile, (row, col));
            }
            _ => {}
        }
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
