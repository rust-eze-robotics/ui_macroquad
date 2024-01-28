use std::{collections::HashMap, rc::Rc, time::Instant};

use macroquad::{
    math::Vec2,
    texture::{load_texture, Texture2D},
};

use crate::core::Drawable;

use self::audio::AudioButton;

use super::icon::IconFactory;

pub mod audio;

#[derive(Debug, PartialEq)]
pub enum ButtonState {
    Active,
    Down,
    Disabled,
    Hovered,
}

pub trait Button: Drawable {}

const BUTTON_ACTIVE_ID: u8 = 0;
const BUTTON_DOWN_ID: u8 = 1;
const BUTTON_DISABLED_ID: u8 = 2;
const BUTTON_HOVERED_ID: u8 = 3;

pub struct ButtonFactory {
    textures: HashMap<u8, Rc<Texture2D>>,
}

impl ButtonFactory {
    pub async fn new() -> Self {
        let mut textures = HashMap::new();

        textures.insert(
            BUTTON_ACTIVE_ID,
            Rc::new(
                load_texture("data/assets/ui/buttons/button_normal.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            BUTTON_DOWN_ID,
            Rc::new(
                load_texture("data/assets/ui/buttons/button_down.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            BUTTON_DISABLED_ID,
            Rc::new(
                load_texture("data/assets/ui/buttons/button_disabled.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            BUTTON_HOVERED_ID,
            Rc::new(
                load_texture("data/assets/ui/buttons/button_hovered.png")
                    .await
                    .unwrap(),
            ),
        );

        Self { textures }
    }

    pub fn new_audio_button(&self, icon_factory: &IconFactory, pos: Vec2) -> AudioButton {
        AudioButton {
            pos,
            size: Vec2::new(64.0, 64.0),
            texture_active: self.textures[&BUTTON_ACTIVE_ID].clone(),
            texture_down: self.textures[&BUTTON_DOWN_ID].clone(),
            texture_disabled: self.textures[&BUTTON_DISABLED_ID].clone(),
            texture_hovered: self.textures[&BUTTON_HOVERED_ID].clone(),
            icon: icon_factory.new_audio_icon(pos),
            state: ButtonState::Active,
            active: true,
        }
    }
}
