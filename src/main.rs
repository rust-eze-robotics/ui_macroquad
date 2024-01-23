pub mod core;
pub mod world;

use macroquad::prelude::*;

#[macroquad::main("Rust-Eze")]
async fn main() {
    loop {
        clear_background(BLACK);

        next_frame().await;
    }
}
