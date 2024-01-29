use std::{collections::HashMap, rc::Rc};

use macroquad::{
    math::Vec2,
    texture::{load_texture, Texture2D},
};

use super::{Icon, IconState};

const AUDIO_ACTIVE_ID: u8 = 0;
const AUDIO_DISABLED_ID: u8 = 1;
const AUDIO_DOWN_ID: u8 = 2;
const CAMERA_ACTIVE_ID: u8 = 3;
const CAMERA_DISABLED_ID: u8 = 4;
const CAMERA_DOWN_ID: u8 = 5;
const SETTINGS_ACTIVE_ID: u8 = 6;
const SETTINGS_DISABLED_ID: u8 = 7;
const SETTINGS_DOWN_ID: u8 = 8;
const SHOP_ACTIVE_ID: u8 = 9;
const SHOP_DISABLED_ID: u8 = 10;
const SHOP_DOWN_ID: u8 = 11;

pub struct IconFactory {
    textures: HashMap<u8, Rc<Texture2D>>,
}

impl IconFactory {
    pub async fn new() -> Self {
        let mut textures = HashMap::new();

        textures.insert(
            AUDIO_ACTIVE_ID,
            Rc::new(
                load_texture("data/assets/ui/icons/audio/audio_active.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            AUDIO_DISABLED_ID,
            Rc::new(
                load_texture("data/assets/ui/icons/audio/audio_disabled.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            AUDIO_DOWN_ID,
            Rc::new(
                load_texture("data/assets/ui/icons/audio/audio_down.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            CAMERA_ACTIVE_ID,
            Rc::new(
                load_texture("data/assets/ui/icons/camera/camera_active.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            CAMERA_DISABLED_ID,
            Rc::new(
                load_texture("data/assets/ui/icons/camera/camera_disabled.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            CAMERA_DOWN_ID,
            Rc::new(
                load_texture("data/assets/ui/icons/camera/camera_down.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            SETTINGS_ACTIVE_ID,
            Rc::new(
                load_texture("data/assets/ui/icons/settings/settings_active.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            SETTINGS_DISABLED_ID,
            Rc::new(
                load_texture("data/assets/ui/icons/settings/settings_disabled.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            SETTINGS_DOWN_ID,
            Rc::new(
                load_texture("data/assets/ui/icons/settings/settings_down.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            SHOP_ACTIVE_ID,
            Rc::new(
                load_texture("data/assets/ui/icons/shop/shop_active.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            SHOP_DISABLED_ID,
            Rc::new(
                load_texture("data/assets/ui/icons/shop/shop_disabled.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            SHOP_DOWN_ID,
            Rc::new(
                load_texture("data/assets/ui/icons/shop/shop_down.png")
                    .await
                    .unwrap(),
            ),
        );

        Self { textures }
    }

    pub fn new_audio_icon(&self, pos: Vec2) -> Icon {
        Icon {
            pos,
            texture_active: self.textures[&AUDIO_ACTIVE_ID].clone(),
            texture_disabled: self.textures[&AUDIO_DISABLED_ID].clone(),
            texture_down: self.textures[&AUDIO_DOWN_ID].clone(),
            state: IconState::Active,
        }
    }

    pub fn new_camera_icon(&self, pos: Vec2) -> Icon {
        Icon {
            pos,
            texture_active: self.textures[&CAMERA_ACTIVE_ID].clone(),
            texture_disabled: self.textures[&CAMERA_DISABLED_ID].clone(),
            texture_down: self.textures[&CAMERA_DOWN_ID].clone(),
            state: IconState::Active,
        }
    }

    pub fn new_settings_icon(&self, pos: Vec2) -> Icon {
        Icon {
            pos,
            texture_active: self.textures[&SETTINGS_ACTIVE_ID].clone(),
            texture_disabled: self.textures[&SETTINGS_DISABLED_ID].clone(),
            texture_down: self.textures[&SETTINGS_DOWN_ID].clone(),
            state: IconState::Active,
        }
    }

    pub fn new_shop_icon(&self, pos: Vec2) -> Icon {
        Icon {
            pos,
            texture_active: self.textures[&SHOP_ACTIVE_ID].clone(),
            texture_disabled: self.textures[&SHOP_DISABLED_ID].clone(),
            texture_down: self.textures[&SHOP_DOWN_ID].clone(),
            state: IconState::Active,
        }
    }
}
