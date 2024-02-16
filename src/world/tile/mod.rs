use crate::core::{context::Context, Drawable};

use super::{content::Content, tile_type::TileType};

pub struct Tile {
    pub tiletype: Box<dyn TileType>,
    pub content: Box<dyn Content>,
    pub visible: bool,
}

impl Tile {
    pub fn new(tiletype: Box<dyn TileType>, content: Box<dyn Content>) -> Self {
        Self {
            tiletype,
            content,
            visible: false,
        }
    }
}

impl Drawable for Tile {
    fn draw(&mut self, context: &Context) {
        if self.visible {
            self.tiletype.draw(context);
            self.content.draw(context);
        }
    }
}
