use macroquad::prelude::*;

use crate::core::{context::Context, Drawable};

pub struct Cursor {
    texture: Texture2D,
}

impl Cursor {
    pub async fn new() -> Self {
        Self {
            texture: load_texture("assets/textures/ui/pointers/cursor.png")
                .await
                .unwrap(),
        }
    }
}

impl Drawable for Cursor {
    fn draw(&mut self, context: &Context) {
        draw_texture(
            &self.texture,
            mouse_position().0 - 22.0,
            mouse_position().1 - 17.0,
            LIGHTGRAY,
        );
    }
}
