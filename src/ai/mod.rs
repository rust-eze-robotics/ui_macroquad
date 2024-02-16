use std::collections::VecDeque;

use pmp_street_picasso::{ToolError, ToolStreetPicasso};
use robotics_lib::{
    energy::Energy,
    event::events::Event,
    interface::{go, robot_map, Direction},
    runner::{backpack::BackPack, Robot, Runnable},
    world::{coordinates::Coordinate, tile::Content, World},
};

use sense_and_find_by_Rustafariani::Lssf;
use spyglass::spyglass::{Spyglass, SpyglassResult};
use ui_lib::RunnableUi;

pub fn is_content_rock(content: &Content) -> bool {
    match content {
        Content::Rock(_) => true,
        _ => false,
    }
}

#[derive(Debug)]
enum State {
    Ready,
    Discover,
    Find,
    Collect,
    Build,
    Terminate,
}

impl Default for State {
    fn default() -> Self {
        State::Ready
    }
}

pub struct BuilderAi {
    ui: Box<dyn RunnableUi>,
    world_size: usize,
    robot: Robot,
    state: State,
    row: usize,
    col: usize,
    rocks: VecDeque<(usize, usize)>,
}

impl BuilderAi {
    pub fn new(ui: Box<dyn RunnableUi>, world_size: usize) -> Self {
        Self {
            ui,
            world_size,
            robot: Robot::new(),
            state: State::Ready,
            row: 0,
            col: 0,
            rocks: VecDeque::new(),
        }
    }

    pub fn run(&mut self, world: &mut World) {
        self.row = self.get_coordinate().get_row();
        self.col = self.get_coordinate().get_col();

        go(self, world, Direction::Down);
        go(self, world, Direction::Right);

        // match self.state {
        //     State::Ready => {
        //         self.do_ready();
        //     }
        //     State::Discover => {
        //         self.do_discover(world);
        //     }
        //     State::Find => {
        //         self.do_find(world);
        //     }
        //     State::Collect => {
        //         self.do_collect(world);
        //     }
        //     State::Build => {
        //         self.do_build(world);
        //     }
        //     State::Terminate => {
        //         self.do_terminate(world);
        //     }
        // }
    }

    fn do_ready(&mut self) {
        self.state = State::Discover;
    }

    fn do_discover(&mut self, world: &mut World) {
        let mut spyglass = Spyglass::new(
            self.row,
            self.col,
            10,
            self.world_size,
            None,
            true,
            1.0,
            |tile| is_content_rock(&tile.content),
        );

        let result = spyglass.new_discover(self, world);

        match result {
            SpyglassResult::Failed(error) => {
                println!("{:?}", error);
            }
            _ => {
                self.state = State::Find;
            }
        }
    }

    fn do_find(&mut self, world: &mut World) {
        let map = robot_map(world).unwrap();

        let mut lssf = Lssf::new();
        lssf.update_map(&map);
        let _ = lssf.update_cost(self.row, self.col);

        self.rocks.extend(lssf.get_content_vec(&Content::Rock(0)));

        if !self.rocks.is_empty() {
            self.state = State::Collect;
        }
    }

    fn do_collect(&mut self, world: &mut World) {
        let map = robot_map(world).unwrap();

        let count = 0;

        if let Some((row, col)) = self.rocks.pop_front() {
            if let Some(tile) = map[row][col].as_ref() {
                let _content = &tile.content;

                // if let Ok(c) =
                //     CollectTool::collect_content(self, world, content, usize::MAX, self.robot.energy.get_energy_level())
                // {
                //     count += c;
                // }
            }
        }

        if count > 0 {
            if self.rocks.is_empty() {
                self.state = State::Build;
            }
        } else {
            if self.rocks.is_empty() {
                self.state = State::Discover;
            }
        }
    }

    fn do_build(&mut self, world: &mut World) {
        let result = ToolStreetPicasso::create_street(self, world, 1, Direction::Right, 3);

        match result {
            Ok(()) => {
                self.state = State::Terminate;
            }
            Err(error) => match error {
                ToolError::NotEnoughMaterial(_) => {
                    self.state = State::Discover;
                }
                _ => {}
            },
        }
    }

    fn do_terminate(&mut self, _world: &World) {}
}

impl Runnable for BuilderAi {
    fn process_tick(&mut self, world: &mut World) {
        self.run(world);
        self.ui.process_tick(world);
    }

    fn handle_event(&mut self, event: Event) {
        self.ui.handle_event(event);
    }

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
