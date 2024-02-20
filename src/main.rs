use core::{
    context::Context, events::EventsHandler, Drawable, TILE_SIZE, WORLD_SCALE, WORLD_SIZE,
    ZOOM_DEFAULT,
};
use std::{cell::RefCell, rc::Rc};

use ai_builder::BuilderAi;
use ai_mcqueen::Ai;
use audio::Audio;
use macroquad::{miniquad::window::set_window_size, prelude::*};
use midgard::{params::WorldGeneratorParameters, WorldGenerator};
use robot::Robot;
use rusteze_ai_artemisia::ArtemisIA;
use wrapper::Wrapper;

use robotics_lib::{runner::Runner, world::world_generator::Generator};
use ui::Ui;
use world::World;

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

    let ai_name = std::env::args()
        .map(|arg| arg.to_owned())
        .collect::<Vec<String>>()
        .get(1)
        .unwrap_or(&String::from("mcqueen"))
        .clone();

    let world_generator_parameters = match ai_name.clone().as_str() {
        "mcqueen" => WorldGeneratorParameters {
            world_size: WORLD_SIZE,
            world_scale: WORLD_SCALE,
            ..ai_mcqueen::get_world_generator_parameters()
        },
        "artemisia" => WorldGeneratorParameters {
            world_size: WORLD_SIZE,
            world_scale: WORLD_SCALE,
            ..rusteze_ai_artemisia::get_world_generator_parameters()
        },
        "builder" => WorldGeneratorParameters {
            world_size: WORLD_SIZE,
            world_scale: WORLD_SCALE,
            ..ai_builder::get_world_generator_parameters()
        },
        _ => panic!("Unknown ai name: {}", ai_name),
    };

    let mut world_generator = WorldGenerator::new(world_generator_parameters);

    let (map, spawn_point, environmental_conditions, _max_score, _score_table) =
        world_generator.gen();

    let robot = Rc::new(RefCell::new(Robot::new(spawn_point).await));

    let world = Rc::new(RefCell::new(
        World::new(&map, environmental_conditions).await,
    ));

    let ui = Rc::new(RefCell::new(Ui::new().await));

    let audio = Rc::new(RefCell::new(Audio::new()));

    let events_handler = Rc::new(RefCell::new(EventsHandler::default()));

    let wrapper = Wrapper::new(
        robot.clone(),
        world.clone(),
        ui.clone(),
        audio.clone(),
        events_handler.clone(),
    );

    let run = match ai_name.clone().as_str() {
        "mcqueen" => Runner::new(Box::new(Ai::new(Box::new(wrapper))), &mut world_generator),
        "artemisia" => Runner::new(
            Box::new(ArtemisIA::new(WORLD_SIZE, Box::new(wrapper))),
            &mut world_generator,
        ),
        "builder" => Runner::new(
            Box::new(BuilderAi::new(Box::new(wrapper), WORLD_SIZE)),
            &mut world_generator,
        ),
        _ => panic!("Unknown ai name: {}", ai_name),
    };

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
            robot
                .borrow_mut()
                .update_state(&context, &mut *world.borrow_mut());

            if robot.borrow().is_ready(&context) {
                if events_handler.borrow().is_empty() {
                    let _ = runner.game_tick();
                }

                events_handler.borrow_mut().handle(
                    runner.get_robot(),
                    &context,
                    &mut *robot.borrow_mut(),
                    &mut *world.borrow_mut(),
                    &mut *ui.borrow_mut(),
                    &mut *audio.borrow_mut(),
                );
            }

            context.update_camera(robot.borrow().get_target_pos(&context));

            ui.borrow_mut().update_gui(&context);
            ui.borrow_mut().handle_input(&context);
            ui.borrow_mut().sync_context(&mut context);
            ui.borrow_mut()
                .sync_character(&mut context, &mut *robot.borrow_mut());

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
