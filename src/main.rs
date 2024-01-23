pub mod core;
pub mod world;

use crate::world::content::{bank::*, coin::*, fire::*, rock::*, tree::*};
use macroquad::prelude::*;
use world::{
    content::{bin::Bin, garbage::Garbage, jollyblock::Jollyblock, market::Market},
    tiletype::grass::Grass,
};

#[macroquad::main("Rust-Eze")]
async fn main() {
    let mut fire = Fire::default();
    fire.load_texture().await;

    let mut bank = Bank::default().await;

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

    let mut market = Market::default();
    market.load_texture().await;

    let mut jollyblock = Jollyblock::default();
    jollyblock.load_texture().await;

    let mut grass = Grass::default();
    grass.load_texture().await;

    loop {
        clear_background(BLACK);

        grass.draw();
        market.draw();

        next_frame().await;
    }
}
