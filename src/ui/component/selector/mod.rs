use std::{
    rc::Rc,
    time::{Duration, Instant},
};

use macroquad::{experimental::animation::AnimatedSprite, prelude::*};

use crate::{
    core::{context::Context, is_released, Drawable},
    robot::character::Character,
    ui::UiItem,
};

pub mod factory;

pub enum SelectorState {
    Idle,
    Selected(Instant),
}

pub struct Selector {
    pub pos: Vec2,
    pub size: Vec2,
    pub character: Rc<dyn Character>,
    pub sprite: AnimatedSprite,
    pub state: SelectorState,
}

impl UiItem for Selector {
    fn update_gui(&mut self) {}

    fn handle_input(&mut self) {
        if is_released(&self.pos, &self.size) {
            self.state = SelectorState::Selected(Instant::now());
            self.sprite = self.character.get_selector_selected_sprite();
        }

        if let SelectorState::Selected(instant) = self.state {
            if instant.elapsed() > Duration::from_millis(1000) {
                self.state = SelectorState::Idle;
                self.sprite = self.character.get_selector_idle_sprite();
            }
        }
    }
}

impl Drawable for Selector {
    fn draw(&mut self, context: &Context) {
        if context.shop_open {
            draw_texture_ex(
                &self.character.get_texture(),
                self.pos.x,
                self.pos.y,
                LIGHTGRAY,
                DrawTextureParams {
                    source: Some(self.sprite.frame().source_rect),
                    dest_size: Some(self.sprite.frame().dest_size),
                    ..Default::default()
                },
            );
        }

        self.sprite.update();
    }
}
