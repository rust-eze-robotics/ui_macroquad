use std::{collections::HashMap, rc::Rc};

use macroquad::prelude::*;

use crate::ui::icon::factory::IconFactory;

use super::{get_current_anchor_position, square::SquareButton, AnchorPosition, ButtonState};

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

    pub fn new_audio_button(&self, icon_factory: &IconFactory) -> SquareButton {
        SquareButton {
            pos: Vec2::default(),
            anchor_pos: AnchorPosition::DOWN_RIGHT(Vec2::new(-64.0, -64.0)),
            size: Vec2::new(64.0, 64.0),
            texture_active: self.textures[&BUTTON_ACTIVE_ID].clone(),
            texture_down: self.textures[&BUTTON_DOWN_ID].clone(),
            texture_disabled: self.textures[&BUTTON_DISABLED_ID].clone(),
            texture_hovered: self.textures[&BUTTON_HOVERED_ID].clone(),
            icon: icon_factory.new_audio_icon(Vec2::default()),
            state: ButtonState::Active,
            active: true,
        }
    }

    pub fn new_camera_button(&self, icon_factory: &IconFactory) -> SquareButton {
        SquareButton {
            pos: Vec2::default(),
            anchor_pos: AnchorPosition::DOWN_RIGHT(Vec2::new(-128.0, -64.0)),
            size: Vec2::new(64.0, 64.0),
            texture_active: self.textures[&BUTTON_ACTIVE_ID].clone(),
            texture_down: self.textures[&BUTTON_DOWN_ID].clone(),
            texture_disabled: self.textures[&BUTTON_DISABLED_ID].clone(),
            texture_hovered: self.textures[&BUTTON_HOVERED_ID].clone(),
            icon: icon_factory.new_camera_icon(Vec2::default()),
            state: ButtonState::Active,
            active: true,
        }
    }

    pub fn new_settings_button(&self, icon_factory: &IconFactory) -> SquareButton {
        SquareButton {
            pos: Vec2::default(),
            anchor_pos: AnchorPosition::TOP_RIGHT(Vec2::new(-64.0, 0.0)),
            size: Vec2::new(64.0, 64.0),
            texture_active: self.textures[&BUTTON_ACTIVE_ID].clone(),
            texture_down: self.textures[&BUTTON_DOWN_ID].clone(),
            texture_disabled: self.textures[&BUTTON_DISABLED_ID].clone(),
            texture_hovered: self.textures[&BUTTON_HOVERED_ID].clone(),
            icon: icon_factory.new_settings_icon(Vec2::default()),
            state: ButtonState::Active,
            active: true,
        }
    }

    pub fn new_shop_button(&self, icon_factory: &IconFactory) -> SquareButton {
        SquareButton {
            pos: Vec2::default(),
            anchor_pos: AnchorPosition::TOP_LEFT(Vec2::new(0.0, 0.0)),
            size: Vec2::new(64.0, 64.0),
            texture_active: self.textures[&BUTTON_ACTIVE_ID].clone(),
            texture_down: self.textures[&BUTTON_DOWN_ID].clone(),
            texture_disabled: self.textures[&BUTTON_DISABLED_ID].clone(),
            texture_hovered: self.textures[&BUTTON_HOVERED_ID].clone(),
            icon: icon_factory.new_shop_icon(Vec2::default()),
            state: ButtonState::Active,
            active: true,
        }
    }
}
