pub mod content;
pub mod decoration;
pub mod tile;
pub mod tile_type;

pub const WORLD_SIZE: usize = 256;
pub const TILE_WIDTH: f32 = 192.0;

use std::collections::HashSet;

use crate::{context::Context, core::Drawable};

use content::Content;
use tile::Tile;
use tile_type::TileType;

use macroquad::math::Vec2;
use robotics_lib::world::{
    environmental_conditions::{self, EnvironmentalConditions},
    tile::{Content as RobContent, Tile as RobTile},
};

use self::{
    content::factory::ContentFactory, decoration::DecorationFactory,
    tile_type::factory::TileTypeFactory,
};

pub struct World {
    pub tiles: Vec<Vec<Tile>>,
    pub environmental_conditions: EnvironmentalConditions,
    hidden_tiles: HashSet<(usize, usize)>,
    size: usize,
    tiletype_factory: TileTypeFactory,
    content_factory: ContentFactory,
    decoration_factory: DecorationFactory,
}

impl World {
    pub async fn new(
        map: &Vec<Vec<RobTile>>,
        environmental_conditions: EnvironmentalConditions,
    ) -> Self {
        let mut ret = Self {
            tiles: Vec::new(),
            environmental_conditions,
            hidden_tiles: HashSet::new(),
            size: map.len(),
            tiletype_factory: TileTypeFactory::new().await,
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

                let tiletype = self
                    .tiletype_factory
                    .from_rob_tile_type(pos, &tile.tile_type);

                let content = self.content_factory.from_rob_content(pos, &tile.content);

                self.tiles[row].push(Tile::new(
                    tiletype,
                    content,
                    self.decoration_factory.new_fog(pos),
                ));
            }
        }
    }

    pub fn update_visibility(&mut self, map: &Vec<Vec<Option<RobTile>>>) {
        for (row, col) in self.hidden_tiles.clone() {
            if map[row][col].is_some() {
                self.tiles[row][col].visible = true;
                self.hidden_tiles.remove(&(row, col));
            }
        }
    }

    pub fn update_tile(&mut self, tile: RobTile, (row, col): (usize, usize)) {
        let pos = Vec2::new(row as f32 * TILE_WIDTH, col as f32 * TILE_WIDTH);

        self.tiles[row][col].tiletype = self
            .tiletype_factory
            .from_rob_tile_type(pos, &tile.tile_type);
        self.tiles[row][col].content = self.content_factory.from_rob_content(pos, &tile.content);
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
