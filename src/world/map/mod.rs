use std::collections::HashSet;

use crate::core::Drawable;

use super::{
    content::{
        bank::Bank, bin::Bin, building::Building, bush::Bush, chest::Chest, coin::Coin, fire::Fire,
        fish::Fish, garbage::Garbage, jollyblock::Jollyblock, market::Market, rock::Rock,
        scarecrow::Scarecrow, tree::Tree, water::Water, Content, ContentFactory,
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
    pub map: Vec<Vec<Tile>>,
    size: usize,
    content_factory: ContentFactory,
}

impl Map {
    pub async fn new(map: &Vec<Vec<RobTile>>) -> Self {
        let mut ret = Self {
            map: Vec::new(),
            size: map.len(),
            content_factory: ContentFactory::new().await,
        };

        ret.setup(map).await;

        ret
    }

    async fn setup(&mut self, map: &Vec<Vec<RobTile>>) {
        for row in 0..self.size {
            self.map.push(Vec::new());

            for col in 0..self.size {
                let tile = &map[row][col];
                let pos = Vec2::new(row as f32 * TILE_WIDTH, col as f32 * TILE_WIDTH);

                let tiletype: Option<Box<dyn Tiletype>>;

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

                let content: Option<Box<dyn Content>>;

                match tile.content {
                    RobContent::Bank(_) => {
                        content = Some(Box::new(self.content_factory.new_bank(pos)));
                    }
                    RobContent::Bin(_) => {
                        content = Some(Box::new(self.content_factory.new_bin(pos)));
                    }
                    RobContent::Building => {
                        content = Some(Box::new(self.content_factory.new_building(pos)));
                    }
                    RobContent::Bush(_) => {
                        content = Some(Box::new(self.content_factory.new_bush(pos)));
                    }
                    RobContent::Coin(_) => {
                        content = Some(Box::new(self.content_factory.new_coin(pos)));
                    }
                    RobContent::Crate(_) => {
                        content = Some(Box::new(self.content_factory.new_rock(pos)));
                    }
                    RobContent::Fire => {
                        content = Some(Box::new(self.content_factory.new_fire(pos)));
                    }
                    RobContent::Fish(_) => {
                        content = Some(Box::new(self.content_factory.new_fish(pos)));
                    }
                    RobContent::Garbage(_) => {
                        content = Some(Box::new(self.content_factory.new_garbage(pos)));
                    }
                    RobContent::JollyBlock(_) => {
                        content = Some(Box::new(self.content_factory.new_jollyblock(pos)));
                    }
                    RobContent::Market(_) => {
                        content = Some(Box::new(self.content_factory.new_market(pos)));
                    }
                    RobContent::Rock(_) => {
                        content = Some(Box::new(self.content_factory.new_rock(pos)));
                    }
                    RobContent::Scarecrow => {
                        content = Some(Box::new(self.content_factory.new_scarecrow(pos)));
                    }
                    RobContent::Tree(_) => {
                        content = Some(Box::new(self.content_factory.new_tree(pos)));
                    }
                    RobContent::Water(_) => {
                        content = Some(Box::new(self.content_factory.new_water(pos)));
                    }
                    RobContent::None => {
                        content = Some(Box::new(self.content_factory.new_none(pos)));
                    }
                }

                if let Some(tiletype) = tiletype {
                    if let Some(content) = content {
                        self.map[row].push(Tile { tiletype, content });
                    }
                }
            }
        }
    }
}

impl Drawable for Map {
    fn draw(&mut self) {
        for row in 0..self.size {
            for col in 0..self.size {
                self.map[row][col].draw();
            }
        }
    }
}
