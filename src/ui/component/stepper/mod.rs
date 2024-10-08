use macroquad::prelude::*;

use crate::{
    core::{context::Context, is_released, Drawable},
    ui::UiItem,
};

use super::clicker::Clicker;

pub mod factory;

pub struct Stepper {
    pub pos: Vec2,
    pub size: Vec2,
    pub title: String,
    pub value: f32,
    pub min_value: f32,
    pub max_value: f32,
    pub increment: f32,
    pub minus_clicker: Clicker,
    pub plus_clicker: Clicker,
}

impl UiItem for Stepper {
    fn update_gui(&mut self) {
        self.minus_clicker.pos = self.pos + Vec2::new(0.0, 0.0);
        self.plus_clicker.pos = self.pos + Vec2::new(self.size.x - self.plus_clicker.size.x, 0.0);
        self.minus_clicker.update_gui();
        self.plus_clicker.update_gui();
    }

    fn handle_input(&mut self) {
        self.minus_clicker.handle_input();
        self.plus_clicker.handle_input();

        if is_released(&self.minus_clicker.pos, &self.minus_clicker.size) {
            self.value -= self.increment;
            self.value = self.value.clamp(self.min_value, self.max_value);
        } else if is_released(&self.plus_clicker.pos, &self.plus_clicker.size) {
            self.value += self.increment;
            self.value = self.value.clamp(self.min_value, self.max_value);
        }
    }
}

impl Drawable for Stepper {
    fn draw(&mut self, context: &Context) {
        let text = format!("{}: {}", self.title, self.value);
        draw_text(&text, self.pos.x + 95.0, self.pos.y + 32.0, 24.0, BLACK);

        self.minus_clicker.draw(context);
        self.plus_clicker.draw(context);
    }
}
