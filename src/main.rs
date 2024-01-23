pub mod world;

use crate::world::fire::*;
use macroquad::prelude::*;

#[macroquad::main("Rust-Eze")]
async fn main() {
    let mut fire = Fire::default();
    fire.load_texture().await;

    loop {
        clear_background(BLACK);

        fire.draw();

        next_frame().await;
    }
}
