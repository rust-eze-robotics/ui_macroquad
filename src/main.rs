use core::Drawable;
use std::{cell::RefCell, rc::Rc, time::Duration};

use ai::{robot::Robot, Ai};
use macroquad::{miniquad::window, prelude::*, time};
use robotics_lib::{
    interface::{robot_map, robot_view},
    runner::{Robot as RobRobot, Runner},
    world::world_generator::Generator,
    world::World as RobWorld,
};
use world::{World, TILE_WIDTH, WORLD_SIZE};

pub mod ai;
pub mod core;
pub mod world;

const CLOCK_MS: u64 = 1000;

#[macroquad::main("Rust-Eze")]
async fn main() {
    window::set_window_size(1600, 900);

    let mut zoom = 0.001;
    let mut target = Vec2::new(0.0, 0.0);
    let mut offset = Vec2::new(0.0, 0.0);

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
        let mut timestamp = std::time::Instant::now();

        loop {
            target = robot.borrow().pos;

            if is_key_down(KeyCode::Left) {
                offset.x += 0.2;
            }
            if is_key_down(KeyCode::Right) {
                offset.x -= 0.2;
            }
            if is_key_down(KeyCode::Up) {
                offset.y -= 0.2;
            }
            if is_key_down(KeyCode::Down) {
                offset.y += 0.2;
            }

            if mouse_wheel().1 != 0.0 {
                zoom *= 1.5f32.powf(mouse_wheel().1);
            }

            clear_background(LIGHTGRAY);

            set_camera(&Camera2D {
                target: target,
                zoom: vec2(zoom, zoom * screen_width() / screen_height()),
                offset: offset,
                ..Default::default()
            });

            if timestamp.elapsed().as_millis() > Duration::from_millis(CLOCK_MS).as_millis() {
                runner.game_tick();
                timestamp = std::time::Instant::now();
            }

            world.borrow_mut().draw();
            robot.borrow_mut().draw();

            // Back to screen space, render some text
            set_default_camera();
            draw_text(
                format!("target (WASD keys) = ({:+.2}, {:+.2})", target.x, target.y).as_str(),
                10.0,
                10.0,
                15.0,
                BLACK,
            );
            draw_text(
                format!(
                    "robot = ({:+.2}, {:+.2})",
                    robot.borrow().pos.x,
                    robot.borrow().pos.y
                )
                .as_str(),
                10.0,
                25.0,
                15.0,
                BLACK,
            );
            draw_text(
                format!("zoom (ctrl + mouse wheel) = {:.2}", zoom).as_str(),
                10.0,
                40.0,
                15.0,
                BLACK,
            );
            draw_text(
                format!("offset (arrow keys) = ({:+.2}, {:+.2})", offset.x, offset.y).as_str(),
                10.0,
                55.0,
                15.0,
                BLACK,
            );

            next_frame().await
        }
    }
}
