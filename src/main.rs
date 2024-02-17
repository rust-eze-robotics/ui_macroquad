use core::{
    context::Context, events::EventsHandler, Drawable, TILE_SIZE, WORLD_SCALE, WORLD_SIZE,
    ZOOM_DEFAULT,
};
use std::{cell::RefCell, rc::Rc};

use audio::Audio;
use macroquad::{miniquad::window::set_window_size, prelude::*};
use robot::Robot;
use wrapper::Wrapper;

use midgard::world_generator::ContentsRadii;
use robotics_lib::{runner::Runner, world::world_generator::Generator};
use ui::Ui;
use world::World;

pub mod ai;
pub mod audio;
pub mod core;
pub mod robot;
pub mod ui;
pub mod world;
pub mod wrapper;

#[macroquad::main("Rust-eze")]
async fn main() {
    set_window_size(900, 900);
    show_mouse(false);

    let params = midgard::world_generator::WorldGeneratorParameters {
        world_size: WORLD_SIZE,
        world_scale: WORLD_SCALE,
        time_progression_minutes: 60,
        contents_radii: ContentsRadii {
            rocks_in_plains: 2,
            rocks_in_hill: 2,
            rocks_in_mountain: 2,
            ..Default::default()
        },
        ..Default::default()
    };

    let mut world_generator = midgard::world_generator::WorldGenerator::new(params);

    let (map, spawn_point, environmental_conditions, _max_score, _score_table) =
        world_generator.gen();

    let robot = Rc::new(RefCell::new(Robot::new(spawn_point).await));

    let world = Rc::new(RefCell::new(
        World::new(&map, environmental_conditions).await,
    ));

    let ui = Rc::new(RefCell::new(Ui::new(world.clone()).await));

    let audio = Rc::new(RefCell::new(Audio::new()));

    let events_handler = Rc::new(RefCell::new(EventsHandler::default()));

    let wrapper = Wrapper::new(
        robot.clone(),
        world.clone(),
        ui.clone(),
        audio.clone(),
        events_handler.clone(),
    );

    let ai = crate::ai::BuilderAi::new(Box::new(wrapper), WORLD_SIZE);

    let run = Runner::new(Box::new(ai), &mut world_generator);

    if let Ok(mut runner) = run {
        let mut context = Context::new(Camera2D {
            target: Vec2::new(
                spawn_point.1 as f32 * TILE_SIZE.x + robot.borrow().offset.x,
                spawn_point.0 as f32 * TILE_SIZE.y + robot.borrow().offset.y,
            ),
            zoom: Vec2::new(ZOOM_DEFAULT, ZOOM_DEFAULT),
            ..Default::default()
        });

        loop {
            robot.borrow_mut().update_state(&context, world.clone());

            if robot.borrow().is_ready(&context) {
                if events_handler.borrow().is_empty() {
                    let _ = runner.game_tick();
                }

                events_handler.borrow_mut().handle(
                    &context,
                    robot.clone(),
                    world.clone(),
                    audio.clone(),
                );
            }

            context.update_camera(robot.borrow().get_target_pos(&context));

            ui.borrow_mut().update_gui(&context);
            ui.borrow_mut().handle_input(&context);
            ui.borrow_mut().sync_context(&mut context);

            clear_background(LIGHTGRAY);

            set_camera(&context.camera());

            world.borrow_mut().draw(&context);
            robot.borrow_mut().draw(&context);

            set_default_camera();

            ui.borrow_mut().draw(&context);

            audio.borrow_mut().play_weather_music(
                &context,
                &world
                    .borrow()
                    .environmental_conditions
                    .get_weather_condition(),
            );

            next_frame().await
        }
    }
}
