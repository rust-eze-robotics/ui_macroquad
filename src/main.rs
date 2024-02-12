use core::{Drawable, CLOCK_MS, ZOOM_DEFAULT};
use std::{cell::RefCell, rc::Rc, time::Duration};

use ai::{robot::Robot, Ai};
use context::Context;
use macroquad::{miniquad::window::set_window_size, prelude::*};

use midgard::world_generator::ContentsRadii;
use robotics_lib::{
    runner::{Robot as RobRobot, Runner},
    world::world_generator::Generator,
};
use ui::Ui;
use world::{World, WORLD_SIZE};

pub mod ai;
pub mod context;
pub mod core;
pub mod ui;
pub mod world;

#[macroquad::main("Rust-Eze")]
async fn main() {
    set_window_size(900, 900);

    // Define the WorldGenerator parameters using the dedicated struct
    let params = midgard::world_generator::WorldGeneratorParameters {
        world_size: WORLD_SIZE,
        contents_radii: ContentsRadii {
            jolly_blocks: 5,
            ..Default::default()
        },
        ..Default::default() // the rest of the parameters keep their default value
    };

    // Instantiate the WorldGenerator with the parameters
    let mut world_generator = midgard::world_generator::WorldGenerator::new(params);

    let (map, spawn_point, _weather, _max_score, _score_table) = world_generator.gen();

    let robot = Rc::new(RefCell::new(Robot::new(spawn_point).await));
    let world = Rc::new(RefCell::new(World::new(&map).await));

    let ai = Ai::new(RobRobot::new(), robot.clone(), world.clone());
    let mut ui = Ui::new(world.clone()).await;

    let run = Runner::new(Box::new(ai), &mut world_generator);

    if let Ok(mut runner) = run {
        let mut context = Context::new(Camera2D {
            target: robot.borrow().get_target_pos(),
            zoom: Vec2::new(ZOOM_DEFAULT, ZOOM_DEFAULT),
            ..Default::default()
        });

        runner.game_tick();

        loop {
            robot.borrow_mut().update_state(&mut context.timestamp);

            if context.timestamp.elapsed().as_millis() > Duration::from_millis(CLOCK_MS).as_millis()
            {
                runner.game_tick();
                context.timestamp = std::time::Instant::now();
            } else {
                context.update_camera(robot.borrow().get_target_pos());

                ui.update_gui();
                ui.handle_input();
                ui.sync_context(&mut context);

                clear_background(LIGHTGRAY);

                set_camera(&context.camera());

                world.borrow_mut().draw(&context);
                robot.borrow_mut().draw(&context);

                set_default_camera();

                ui.draw(&context);

                next_frame().await
            }
        }
    }
}
