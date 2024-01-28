pub mod content;
pub mod decoration;
pub mod tile;
pub mod tiletype;

pub const WORLD_SIZE: usize = 256;
pub const TILE_WIDTH: f32 = 192.0;

use std::collections::HashSet;

use crate::{context::Context, core::Drawable};

use content::{Content, ContentFactory};
use tile::Tile;
use tiletype::{Tiletype, TiletypeFactory};

use macroquad::math::Vec2;
use robotics_lib::world::tile::{Content as RobContent, Tile as RobTile, TileType as RobTiletype};

use self::decoration::DecorationFactory;

pub struct World {
    pub tiles: Vec<Vec<Tile>>,
    hidden_tiles: HashSet<(usize, usize)>,
    size: usize,
    tiletype_factory: TiletypeFactory,
    content_factory: ContentFactory,
    decoration_factory: DecorationFactory,
}

impl World {
    pub async fn new(map: &Vec<Vec<RobTile>>) -> Self {
        let mut ret = Self {
            tiles: Vec::new(),
            hidden_tiles: HashSet::new(),
            size: map.len(),
            tiletype_factory: TiletypeFactory::new().await,
            content_factory: ContentFactory::new().await,
            decoration_factory: DecorationFactory::new().await,
        };

        ret.setup(map).await;

        ret
    }

    async fn setup(&mut self, map: &Vec<Vec<RobTile>>) {
        for row in 0..self.size {
            self.tiles.push(Vec::new());

            for col in 0..self.size {
                self.hidden_tiles.insert((row, col));

                let tile = &map[row][col];
                let pos = Vec2::new(col as f32 * TILE_WIDTH, row as f32 * TILE_WIDTH);

                let tiletype: Option<Box<dyn Tiletype>>;

                match tile.tile_type {
                    RobTiletype::DeepWater => {
                        tiletype = Some(Box::new(self.tiletype_factory.new_deep_water(pos)));
                    }
                    RobTiletype::Grass => {
                        tiletype = Some(Box::new(self.tiletype_factory.new_grass(pos)));
                    }
                    RobTiletype::Hill => {
                        tiletype = Some(Box::new(self.tiletype_factory.new_hill(pos)));
                    }
                    RobTiletype::Lava => {
                        tiletype = Some(Box::new(self.tiletype_factory.new_lava(pos)));
                    }
                    RobTiletype::Sand => {
                        tiletype = Some(Box::new(self.tiletype_factory.new_mountain(pos)));
                    }
                    RobTiletype::Mountain => {
                        tiletype = Some(Box::new(self.tiletype_factory.new_sand(pos)));
                    }
                    RobTiletype::ShallowWater => {
                        tiletype = Some(Box::new(self.tiletype_factory.new_shallow_water(pos)));
                    }
                    RobTiletype::Snow => {
                        tiletype = Some(Box::new(self.tiletype_factory.new_snow(pos)));
                    }
                    RobTiletype::Street => {
                        tiletype = Some(Box::new(self.tiletype_factory.new_street(pos)));
                    }
                    RobTiletype::Teleport(_) => {
                        tiletype = Some(Box::new(self.tiletype_factory.new_teleport(pos)));
                    }
                    RobTiletype::Wall => {
                        tiletype = Some(Box::new(self.tiletype_factory.new_wall(pos)));
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
                        content = Some(Box::new(self.content_factory.new_chest(pos)));
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
                        self.tiles[row].push(Tile::new(
                            tiletype,
                            content,
                            self.decoration_factory.new_fog(pos),
                        ));
                    }
                }
            }
        }
    }

    pub fn update(&mut self, map: &Vec<Vec<Option<RobTile>>>) {
        for (row, col) in self.hidden_tiles.clone() {
            if map[row][col].is_some() {
                self.tiles[row][col].visible = true;
                self.hidden_tiles.remove(&(row, col));
            }
        }
    }
}

impl Drawable for World {
    fn draw(&mut self, context: &Context) {
        for row in 0..self.size {
            for col in 0..self.size {
                self.tiles[row][col].draw(context);
            }
        }
    }
}
