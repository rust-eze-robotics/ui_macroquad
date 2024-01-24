use crate::core::Drawable;

use super::{content::Content, tiletype::Tiletype};

pub struct Tile {
    pub tiletype: Box<dyn Tiletype>,
    pub content: Box<dyn Content>,
}

impl Drawable for Tile {
    fn draw(&mut self) {
        self.tiletype.draw();
        self.content.draw();
    }
}
