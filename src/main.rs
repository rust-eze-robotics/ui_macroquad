pub mod world;

use crate::world::contents::{bank::*, fire::*};
use macroquad::prelude::*;

#[macroquad::main("Rust-Eze")]
async fn main() {
    let mut fire = Fire::default();
    fire.load_texture().await;

    let mut bank = Bank::default();
    bank.load_texture().await;

    loop {
        clear_background(BLACK);

        bank.draw();

        next_frame().await;
    }
}
