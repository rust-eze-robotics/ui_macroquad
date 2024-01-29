use crate::{context::Context, core::Drawable};

use super::{content::Content, decoration::fog::Fog, tile_type::TileType};

pub struct Tile {
    pub tiletype: Box<dyn TileType>,
    pub content: Box<dyn Content>,
    pub fog: Fog,
    pub visible: bool,
}

impl Tile {
    pub fn new(tiletype: Box<dyn TileType>, content: Box<dyn Content>, fog: Fog) -> Self {
        Self {
            tiletype,
            content,
            fog,
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
