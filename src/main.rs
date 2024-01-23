pub mod world;

use crate::world::contents::{bank::*, coin::*, fire::*, rock::*, tree::*};
use macroquad::prelude::*;
use world::contents::{bin::Bin, garbage::Garbage, jollyblock::Jollyblock};

#[macroquad::main("Rust-Eze")]
async fn main() {
    let mut fire = Fire::default();
    fire.load_texture().await;

    let mut bank = Bank::default();
    bank.load_texture().await;

    let mut tree = Tree::default();
    tree.load_texture().await;

    let mut coin = Coin::default();
    coin.load_texture().await;

    let mut rock = Rock::default();
    rock.load_texture().await;

    let mut garbage = Garbage::default();
    garbage.load_texture().await;

    let mut bin = Bin::default();
    bin.load_texture().await;

    let mut jollyblock = Jollyblock::default();
    jollyblock.load_texture().await;

    loop {
        clear_background(BLACK);

        jollyblock.draw();

        next_frame().await;
    }
}
