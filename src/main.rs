use core::Drawable;
use std::{any::Any, cell::RefCell, os::unix::raw::time_t, rc::Rc, time::Duration};

use ai::{
    robot::{Robot, State},
    Ai,
};
use context::{Context, CLOCK_MS};
use macroquad::{experimental::camera::mouse::Camera, miniquad::window, prelude::*, time};
use midgard::world_generator::ContentsRadii;
use robotics_lib::{
    interface::{robot_map, robot_view},
    runner::{Robot as RobRobot, Runner},
    world::World as RobWorld,
    world::{tile::Content, world_generator::Generator},
};
use world::{World, TILE_WIDTH, WORLD_SIZE};

pub mod ai;
pub mod context;
pub mod core;
pub mod world;

#[macroquad::main("Rust-Eze")]
async fn main() {
    window::set_window_size(900, 900);

    // Define the WorldGenerator parameters using the dedicated struct
    let params = midgard::world_generator::WorldGeneratorParameters {
        world_size: WORLD_SIZE,
        ..Default::default() // the rest of the parameters keep their default value
    };

    // Instantiate the WorldGenerator with the parameters
    let mut world_generator = midgard::world_generator::WorldGenerator::new(params);

    let (map, spawn_point, _weather, _max_score, _score_table) = world_generator.gen();

    let robot = Rc::new(RefCell::new(Robot::new(spawn_point).await));
    let world = Rc::new(RefCell::new(World::new(&map).await));

    let ai = Ai {
        rob_robot: RobRobot::new(),
        robot: robot.clone(),
        world: world.clone(),
    };

    let run = Runner::new(Box::new(ai), &mut world_generator);

    if let Ok(mut runner) = run {
        let mut context = Context::new(robot.borrow().get_target_pos(), Vec2::default());

        runner.game_tick();

        loop {
            robot.borrow_mut().update_state(&mut context.timestamp);

            if context.timestamp.elapsed().as_millis() > Duration::from_millis(CLOCK_MS).as_millis()
            {
                runner.game_tick();
                context.timestamp = std::time::Instant::now();
            } else {
                context.camera.target = robot.borrow().get_target_pos();

                context.update_camera();

                clear_background(LIGHTGRAY);

                set_camera(&context.camera());

                world.borrow_mut().draw(&context);
                robot.borrow_mut().draw(&context);

                set_default_camera();

                next_frame().await
            }
        }
    }
}
