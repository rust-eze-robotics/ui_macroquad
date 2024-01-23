use super::{content::Content, tiletype::Tiletype};

pub struct Tile {
    pub tiletype: Box<dyn Tiletype>,
    pub content: Box<dyn Content>,
}
