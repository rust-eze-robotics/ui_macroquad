use std::{cell::RefCell, rc::Rc};

use robotics_lib::{
    energy::Energy,
    event::events::Event,
    interface::{go, robot_map},
    runner::{backpack::BackPack, Robot as RobRobot, Runnable},
    world::{coordinates::Coordinate, World as RobWorld},
};

use crate::world::World;

use self::robot::Robot;

pub mod robot;

pub struct Ai {
    pub rob_robot: RobRobot,
    pub robot: Rc<RefCell<Robot>>,
    pub world: Rc<RefCell<World>>,
}

impl Runnable for Ai {
    fn process_tick(&mut self, world: &mut RobWorld) {
        let map = robot_map(world).unwrap();

        self.robot.borrow_mut().update_pos(
            self.get_coordinate().get_col(),
            self.get_coordinate().get_row(),
        );
        self.world.borrow_mut().update(&map);
    }

    fn handle_event(&mut self, event: Event) {
        println!();
        println!("{:?}", event);
        println!();
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
