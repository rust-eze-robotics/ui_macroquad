use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use macroquad::math::Vec2;
use robotics_lib::{event::events::Event, runner::Runnable};

use crate::{audio::Audio, robot::Robot, ui::Ui, world::World};

use super::{context::Context, TILE_SIZE};

#[derive(Debug, Default)]
pub struct EventsHandler {
    events: VecDeque<Event>,
}

impl EventsHandler {
    pub fn push(&mut self, event: Event) {
        self.events.push_back(event);
    }

    pub fn peek(&mut self) -> Option<&Event> {
        self.events.front()
    }

    pub fn pop(&mut self) -> Option<Event> {
        self.events.pop_front()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    pub fn handle(
        &mut self,
        runnable: &Box<dyn Runnable>,
        context: &Context,
        robot: Rc<RefCell<Robot>>,
        world: Rc<RefCell<World>>,
        ui: Rc<RefCell<Ui>>,
        audio: Rc<RefCell<Audio>>,
    ) {
        while !self.is_empty() {
            let event = self.pop().unwrap();

            audio.borrow_mut().play_event_sound(context, &event);

            match event {
                Event::DayChanged(environmental_conditions) => {
                    world.borrow_mut().environmental_conditions = environmental_conditions;
                }
                Event::EnergyConsumed(_amount) => {
                    ui.borrow_mut()
                        .energy_bar
                        .update_energy(runnable.get_energy().get_energy_level());
                }
                Event::EnergyRecharged(_amount) => {
                    ui.borrow_mut()
                        .energy_bar
                        .update_energy(runnable.get_energy().get_energy_level());
                }
                Event::Moved(_tile, (row, col)) => {
                    let new_pos = Vec2::new(col as f32 * TILE_SIZE.x, row as f32 * TILE_SIZE.y);

                    if robot.borrow().pos.distance(new_pos) >= TILE_SIZE.x * 2.0 {
                        robot.borrow_mut().set_teleport(context, new_pos);
                    } else {
                        robot.borrow_mut().set_walk(context, new_pos);
                    }

                    robot.borrow_mut().update_orientation(new_pos);

                    return;
                }
                Event::TileContentUpdated(tile, (row, col)) => {
                    let pos = Vec2::new(col as f32 * TILE_SIZE.x, row as f32 * TILE_SIZE.y);

                    robot.borrow_mut().set_interact(context, pos, tile);
                    robot.borrow_mut().update_orientation(pos);

                    return;
                }
                Event::TimeChanged(environmental_conditions) => {
                    world.borrow_mut().environmental_conditions = environmental_conditions;
                }
                _ => {}
            }
        }
    }
}
