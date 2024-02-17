use std::rc::Rc;

use macroquad::prelude::*;

use crate::core::{context::Context, get_current_anchor_position, AnchorPosition, Drawable};

use self::factory::BarFactory;

use super::UiItem;

pub mod factory;

pub struct Bar {
    pos: Vec2,
    anchor_pos: AnchorPosition,
    texture: Rc<Texture2D>,
    bar_factory: BarFactory,
}

impl Bar {
    pub async fn new() -> Self {
        Self {
            pos: Vec2::default(),
            anchor_pos: AnchorPosition::DownLeft(Vec2::new(0.0, -90.0)),
            texture: Rc::new(Texture2D::empty()),
            bar_factory: BarFactory::new().await,
        }
    }

    pub fn update_character(&mut self) {}

    pub fn update_energy(&mut self, energy: usize) {
        self.texture = self.bar_factory.get_torch_texture(energy);
    }
}

impl UiItem for Bar {
    fn update_gui(&mut self) {
        self.pos = get_current_anchor_position(self.anchor_pos);
    }

    fn handle_input(&mut self) {}
}

impl Drawable for Bar {
    fn draw(&mut self, _context: &Context) {
        draw_texture(&self.texture, self.pos.x, self.pos.y, LIGHTGRAY);
    }
}
