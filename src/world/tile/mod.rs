use macroquad::camera::Camera2D;

use crate::core::Drawable;

use super::{content::Content, tiletype::Tiletype};

pub struct Tile {
    pub tiletype: Box<dyn Tiletype>,
    pub content: Box<dyn Content>,
    pub visible: bool,
}

impl Drawable for Tile {
    fn draw(&mut self, camera: &Camera2D) {
        if self.visible {
            self.tiletype.draw(camera);
            self.content.draw(camera);
        }
    }
}
