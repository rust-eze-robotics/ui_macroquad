use robotics_lib::{
    energy::Energy,
    event::events::Event,
    interface::{robot_map, robot_view},
    runner::{backpack::BackPack, Robot, Runnable},
    world::{coordinates::Coordinate, World},
};

use crate::{core::Drawable, world::map::Map};

pub struct MyRobot {
    pub robot: Robot,
    pub map: Map,
}

impl Runnable for MyRobot {
    fn process_tick(&mut self, world: &mut World) {
        self.map.draw();
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
