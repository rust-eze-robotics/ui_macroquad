use std::{cell::RefCell, collections::VecDeque, rc::Rc, time::Instant};

use macroquad::math::Vec2;
use robotics_lib::event::events::Event;

use crate::{
    audio::Audio,
    robot::{Robot, RobotState},
    world::World,
};

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
        context: &Context,
        robot: Rc<RefCell<Robot>>,
        world: Rc<RefCell<World>>,
        audio: Rc<RefCell<Audio>>,
    ) {
        while !self.is_empty() {
            let event = self.pop().unwrap();

            audio.borrow_mut().play_event_sound(context, &event);

            match event {
                Event::DayChanged(environmental_conditions) => {
                    world.borrow_mut().environmental_conditions = environmental_conditions;
                }
                Event::EnergyConsumed(amount) => {
                    robot.borrow_mut().energy -= amount;
                }
                Event::EnergyRecharged(amount) => {
                    robot.borrow_mut().energy += amount;
                }
                Event::Moved(_tile, (row, col)) => {
                    let new_pos = Vec2::new(col as f32 * TILE_SIZE.x, row as f32 * TILE_SIZE.y);

                    if robot.borrow().pos.distance(new_pos) >= TILE_SIZE.x * 2.0 {
                        robot.borrow_mut().set_teleport(new_pos);
                    } else {
                        robot.borrow_mut().set_walk(new_pos);
                    }

                    robot.borrow_mut().update_orientation(new_pos);

                    return;
                }
                Event::TileContentUpdated(tile, (row, col)) => {
                    let vec = Vec2::new(col as f32 * TILE_SIZE.x, row as f32 * TILE_SIZE.y);

                    robot.borrow_mut().set_interact(vec);
                    world.borrow_mut().update_tile(tile, (row, col));

                    robot.borrow_mut().update_orientation(vec);

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
