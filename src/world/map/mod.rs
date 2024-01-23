use std::collections::HashSet;

use super::{
    content::{
        bank::Bank, bin::Bin, building::Building, bush::Bush, chest::Chest, coin::Coin, fire::Fire,
        fish::Fish, garbage::Garbage, jollyblock::Jollyblock, market::Market, rock::Rock,
        scarecrow::Scarecrow, tree::Tree, water::Water, Content,
    },
    tile::Tile,
    tiletype::{
        deep_water::DeepWater, grass::Grass, hill::Hill, lava::Lava, mountain::Mountain,
        sand::Sand, shallow_water::ShallowWater, snow::Snow, street::Street, teleport::Teleport,
        wall::Wall, Tiletype,
    },
};
use macroquad::math::Vec2;
use robotics_lib::world::tile::{Content as RobContent, Tile as RobTile, TileType as RobTiletype};

const TILE_WIDTH: f32 = 192.0;

pub struct Map {
    pub map: Vec<Vec<Option<Tile>>>,
    size: usize,
    hidden: HashSet<(usize, usize)>,
}

impl Map {
    fn new(size: usize) -> Self {
        let mut map = Vec::new();
        let mut hidden = HashSet::new();

        for row in 0..size {
            map.push(Vec::new());

            for col in 0..size {
                map[row].push(None);
                hidden.insert((row, col));
            }
        }

        Self { map, size, hidden }
    }

    pub async fn update(&mut self, map: &Vec<Vec<Option<RobTile>>>) {
        for (row, col) in &self.hidden {
            if let Some(tile) = map[*row][*col].as_ref() {
                let pos = Vec2::new(*row as f32 * TILE_WIDTH, *col as f32 * TILE_WIDTH);

                let mut tiletype: Option<Box<dyn Tiletype>> = None;

                match tile.tile_type {
                    RobTiletype::DeepWater => {
                        tiletype = Some(Box::new(DeepWater::new(pos).await));
                    }
                    RobTiletype::Grass => {
                        tiletype = Some(Box::new(Grass::new(pos).await));
                    }
                    RobTiletype::Hill => {
                        tiletype = Some(Box::new(Hill::new(pos).await));
                    }
                    RobTiletype::Lava => {
                        tiletype = Some(Box::new(Lava::new(pos).await));
                    }
                    RobTiletype::Sand => {
                        tiletype = Some(Box::new(Sand::new(pos).await));
                    }
                    RobTiletype::Mountain => {
                        tiletype = Some(Box::new(Mountain::new(pos).await));
                    }
                    RobTiletype::ShallowWater => {
                        tiletype = Some(Box::new(ShallowWater::new(pos).await));
                    }
                    RobTiletype::Snow => {
                        tiletype = Some(Box::new(Snow::new(pos).await));
                    }
                    RobTiletype::Street => {
                        tiletype = Some(Box::new(Street::new(pos).await));
                    }
                    RobTiletype::Teleport(_) => {
                        tiletype = Some(Box::new(Teleport::new(pos).await));
                    }
                    RobTiletype::Wall => {
                        tiletype = Some(Box::new(Wall::new(pos).await));
                    }
                }

                let mut content: Option<Box<dyn Content>> = None;

                match tile.content {
                    RobContent::Bank(_) => {
                        content = Some(Box::new(Bank::new(pos).await));
                    }
                    RobContent::Bin(_) => {
                        content = Some(Box::new(Bin::new(pos).await));
                    }
                    RobContent::Building => {
                        content = Some(Box::new(Building::new(pos).await));
                    }
                    RobContent::Bush(_) => {
                        content = Some(Box::new(Bush::new(pos).await));
                    }
                    RobContent::Coin(_) => {
                        content = Some(Box::new(Coin::new(pos).await));
                    }
                    RobContent::Crate(_) => {
                        content = Some(Box::new(Chest::new(pos).await));
                    }
                    RobContent::Fire => {
                        content = Some(Box::new(Fire::new(pos).await));
                    }
                    RobContent::Fish(_) => {
                        content = Some(Box::new(Fish::new(pos).await));
                    }
                    RobContent::Garbage(_) => {
                        content = Some(Box::new(Garbage::new(pos).await));
                    }
                    RobContent::JollyBlock(_) => {
                        content = Some(Box::new(Jollyblock::new(pos).await));
                    }
                    RobContent::Market(_) => {
                        content = Some(Box::new(Market::new(pos).await));
                    }
                    RobContent::Rock(_) => {
                        content = Some(Box::new(Rock::new(pos).await));
                    }
                    RobContent::Scarecrow => {
                        content = Some(Box::new(Scarecrow::new(pos).await));
                    }
                    RobContent::Tree(_) => {
                        content = Some(Box::new(Tree::new(pos).await));
                    }
                    RobContent::Water(_) => {
                        content = Some(Box::new(Water::new(pos).await));
                    }
                    RobContent::None => {
                        content = Some(Box::new(super::content::none::None::new(pos).await));
                    }
                }
            }
        }
    }
}
