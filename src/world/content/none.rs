use crate::core::{context::Context, Drawable};

use super::Content;

pub struct None {}

impl Content for None {}

impl Drawable for None {
    fn draw(&mut self, _context: &Context) {}
}
