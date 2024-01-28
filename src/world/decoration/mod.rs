use std::{collections::HashMap, rc::Rc};

use macroquad::{
    math::Vec2,
    texture::{load_texture, Texture2D},
};

use crate::core::Drawable;

use self::fog::Fog;

pub mod fog;

pub trait Decoration: Drawable {}

const FOG_ID: u8 = 0;

pub struct DecorationFactory {
    textures: HashMap<u8, Rc<Texture2D>>,
}

impl DecorationFactory {
    pub async fn new() -> Self {
        let mut textures = HashMap::new();

        textures.insert(
            FOG_ID,
            Rc::new(
                load_texture("data/assets/decorations/fog/fog.png")
                    .await
                    .unwrap(),
            ),
        );

        Self { textures }
    }

    pub fn new_fog(&self, pos: Vec2) -> Fog {
        Fog {
            pos,
            offset: Vec2::new(0.0, 0.0),
            texture: self.textures[&FOG_ID].clone(),
        }
    }
}
