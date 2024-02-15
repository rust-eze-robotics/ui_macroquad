use std::{cell::RefCell, collections::VecDeque, rc::Rc, time::Instant};

use macroquad::math::Vec2;
use robotics_lib::event::events::Event;

use crate::{
    audio::Audio,
    robot::{Robot, RobotState},
    world::World,
};

use super::TILE_SIZE;

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
        robot: Rc<RefCell<Robot>>,
        world: Rc<RefCell<World>>,
        audio: Rc<RefCell<Audio>>,
    ) {
        while !self.is_empty() {
            let event = self.pop().unwrap();

            audio.borrow_mut().play_event(&event);

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
                        robot.borrow_mut().state = RobotState::Teleporting(Instant::now(), new_pos);
                    } else {
                        robot.borrow_mut().state = RobotState::Walking(Instant::now(), new_pos);
                    }

                    return;
                }
                Event::TileContentUpdated(tile, (row, col)) => {
                    let new_pos = Vec2::new(col as f32 * TILE_SIZE.x, row as f32 * TILE_SIZE.y);

                    robot.borrow_mut().state = RobotState::Interacting(Instant::now(), new_pos);
                    world.borrow_mut().update_tile(tile, (row, col));

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
