use super::{content::Content, tiletype::Tiletype};

pub struct Tile {
    tiletype: Box<dyn Tiletype>,
    content: Box<dyn Content>,
}
