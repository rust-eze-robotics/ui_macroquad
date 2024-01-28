use std::{collections::HashMap, rc::Rc};

use macroquad::{
    math::Vec2,
    texture::{load_texture, Texture2D},
};

use crate::core::Drawable;

use self::audio::AudioIcon;

pub mod audio;

#[derive(Debug, PartialEq)]
pub enum IconState {
    Active,
    Disabled,
    Down,
}

pub trait Icon: Drawable {}

const AUDIO_ACTIVE_ID: u8 = 0;
const AUDIO_DISABLED_ID: u8 = 1;
const AUDIO_DOWN_ID: u8 = 2;
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

        Self { textures }
    }

    pub fn new_audio_icon(&self, pos: Vec2) -> AudioIcon {
        AudioIcon {
            pos,
            texture_active: self.textures[&AUDIO_ACTIVE_ID].clone(),
            texture_disabled: self.textures[&AUDIO_DISABLED_ID].clone(),
            texture_down: self.textures[&AUDIO_DOWN_ID].clone(),
            state: IconState::Active,
        }
    }
}
