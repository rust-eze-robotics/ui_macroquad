use core::Drawable;
use std::{cell::RefCell, rc::Rc, time::Duration};

use ai::{robot::Robot, Ai};
use macroquad::{experimental::camera::mouse::Camera, miniquad::window, prelude::*, time};
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
const ZOOM_MIN: f32 = 0.001;
const ZOOM_MAX: f32 = 0.0034;
const ZOOM_DEFAULT: f32 = 0.0015;

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
        let mut timestamp = std::time::Instant::now();
        let mut camera = Camera2D {
            zoom: vec2(ZOOM_DEFAULT, ZOOM_DEFAULT),
            ..Default::default()
        };

        runner.game_tick();

        loop {
            if timestamp.elapsed().as_millis() > Duration::from_millis(CLOCK_MS).as_millis() {
                runner.game_tick();
                timestamp = std::time::Instant::now();
            } else {
                camera.target.x = robot.borrow().pos.x + TILE_WIDTH / 2.0;
                camera.target.y = robot.borrow().pos.y + TILE_WIDTH / 2.0;

                if is_key_down(KeyCode::Left) {
                    camera.offset.x += 0.2;
                }
                if is_key_down(KeyCode::Right) {
                    camera.offset.x -= 0.2;
                }
                if is_key_down(KeyCode::Up) {
                    camera.offset.y -= 0.2;
                }
                if is_key_down(KeyCode::Down) {
                    camera.offset.y += 0.2;
                }

                if is_key_down(KeyCode::Space) {
                    camera.offset.x = 0.0;
                    camera.offset.y = 0.0;
                }

                if mouse_wheel().1 != 0.0 {
                    camera.zoom.x *= 1.5f32.powf(mouse_wheel().1);
                    if !is_key_down(KeyCode::LeftControl) {
                        camera.zoom.x = clamp(camera.zoom.x, ZOOM_MIN, ZOOM_MAX);
                    }
                    camera.zoom.y = camera.zoom.x * screen_width() / screen_height();
                }

                clear_background(LIGHTGRAY);

                set_camera(&camera);

                world.borrow_mut().draw(&camera);
                robot.borrow_mut().draw(&camera);

                println!();

                // Back to screen space, render some text
                set_default_camera();
                draw_text(
                    format!(
                        "camera.target (WASD keys) = ({:+.2}, {:+.2})",
                        camera.target.x, camera.target.y
                    )
                    .as_str(),
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
                    format!("zoom (ctrl + mouse wheel) = {}", camera.zoom.x).as_str(),
                    10.0,
                    40.0,
                    15.0,
                    BLACK,
                );
                draw_text(
                    format!(
                        "camera.offset (arrow keys) = ({:+.2}, {:+.2})",
                        camera.offset.x, camera.offset.y
                    )
                    .as_str(),
                    10.0,
                    55.0,
                    15.0,
                    BLACK,
                );

                next_frame().await
            }
        }
    }
}
