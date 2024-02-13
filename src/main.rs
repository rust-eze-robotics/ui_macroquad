use core::{context::Context, events::EventsHandler, Drawable, ZOOM_DEFAULT};
use std::{cell::RefCell, rc::Rc};

use macroquad::{miniquad::window::set_window_size, prelude::*};
use robot::Robot;
use wrapper::Wrapper;

use midgard::world_generator::ContentsRadii;
use robotics_lib::{
    runner::{Robot as RobRobot, Runner},
    world::world_generator::Generator,
};
use ui::Ui;
use world::{World, TILE_WIDTH, WORLD_SIZE};

pub mod core;
pub mod robot;
pub mod ui;
pub mod world;
pub mod wrapper;

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

    let (map, spawn_point, environmental_conditions, _max_score, _score_table) =
        world_generator.gen();

    let robot = Rc::new(RefCell::new(Robot::new(spawn_point).await));

    let world = Rc::new(RefCell::new(
        World::new(&map, environmental_conditions).await,
    ));

    let events_handler = Rc::new(RefCell::new(EventsHandler::default()));

    let wrapper = Wrapper::new(
        RobRobot::new(),
        robot.clone(),
        world.clone(),
        events_handler.clone(),
    );

    let mut ui = Ui::new(world.clone()).await;

    let run = Runner::new(Box::new(wrapper), &mut world_generator);

    if let Ok(mut runner) = run {
        let mut context = Context::new(Camera2D {
            target: Vec2::new(
                spawn_point.1 as f32 * TILE_WIDTH + robot.borrow().offset.x,
                spawn_point.0 as f32 * TILE_WIDTH + robot.borrow().offset.y,
            ),
            zoom: Vec2::new(ZOOM_DEFAULT, ZOOM_DEFAULT),
            ..Default::default()
        });

        loop {
            robot.borrow_mut().update_state(&context);

            if robot.borrow().ready(&context) {
                if events_handler.borrow().is_empty() {
                    runner.game_tick();
                }

                events_handler
                    .borrow_mut()
                    .handle(robot.clone(), world.clone());
            }

            context.update_camera(robot.borrow().get_target_pos(&context));

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
