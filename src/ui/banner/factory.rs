use std::{collections::HashMap, rc::Rc};

use macroquad::{
    math::Vec2,
    texture::{load_texture, Texture2D},
};

use super::{HorizontalBanner, VerticalBanner};

const HORIZONTAL_LEFT_ID: u8 = 0;
const HORIZONTAL_CENTER_ID: u8 = 1;
const HORIZONTAL_RIGHT_ID: u8 = 2;
const VERTICAL_UP_ID: u8 = 3;
const VERTICAL_CENTER_ID: u8 = 4;
const VERTICAL_DOWN_ID: u8 = 5;

pub struct BannerFactory {
    textures: HashMap<u8, Rc<Texture2D>>,
}

impl BannerFactory {
    pub async fn new() -> Self {
        let mut textures = HashMap::new();

        textures.insert(
            HORIZONTAL_LEFT_ID,
            Rc::new(
                load_texture("data/assets/ui/banners/horizontal/banner_left.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            HORIZONTAL_CENTER_ID,
            Rc::new(
                load_texture("data/assets/ui/banners/horizontal/banner_center.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            HORIZONTAL_RIGHT_ID,
            Rc::new(
                load_texture("data/assets/ui/banners/horizontal/banner_right.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            VERTICAL_UP_ID,
            Rc::new(
                load_texture("data/assets/ui/banners/vertical/banner_up.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            VERTICAL_CENTER_ID,
            Rc::new(
                load_texture("data/assets/ui/banners/vertical/banner_center.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            VERTICAL_DOWN_ID,
            Rc::new(
                load_texture("data/assets/ui/banners/vertical/banner_down.png")
                    .await
                    .unwrap(),
            ),
        );

        Self { textures }
    }

    pub fn new_horizzontal_banner(&self, pos: Vec2) -> HorizontalBanner {
        HorizontalBanner {
            pos,
            texture_left: self.textures[&HORIZONTAL_LEFT_ID].clone(),
            texture_center: self.textures[&HORIZONTAL_CENTER_ID].clone(),
            texture_right: self.textures[&HORIZONTAL_RIGHT_ID].clone(),
        }
    }

    pub fn new_vertical_banner(&self, pos: Vec2) -> VerticalBanner {
        VerticalBanner {
            pos,
            texture_up: self.textures[&VERTICAL_UP_ID].clone(),
            texture_center: self.textures[&VERTICAL_CENTER_ID].clone(),
            texture_down: self.textures[&VERTICAL_DOWN_ID].clone(),
        }
    }
}
