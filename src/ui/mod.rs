use std::{cell::RefCell, rc::Rc};

use macroquad::{
    math::Vec2,
    window::{screen_height, screen_width},
};

use crate::{core::context::Context, core::Drawable, world::World};

use self::{
    banner::{factory::BannerFactory, HorizontalBanner},
    button::{button::Button, factory::ButtonFactory},
    icon::factory::IconFactory,
    map::Map,
};

pub mod banner;
pub mod button;
pub mod icon;
pub mod map;

pub trait UiComponent: Drawable {
    fn update_gui(&mut self);
    fn handle_input(&mut self);
}

pub struct Ui {
    pub map: Map,
    audio_button: Button,
    camera_button: Button,
    settings_button: Button,
    shop_button: Button,
    banner: HorizontalBanner,
}

impl Ui {
    pub fn update_gui(&mut self) {
        self.audio_button.update_gui();
        self.camera_button.update_gui();
        self.settings_button.update_gui();
        self.shop_button.update_gui();
    }

    pub fn handle_input(&mut self) {
        self.audio_button.handle_input();
        self.camera_button.handle_input();
        self.settings_button.handle_input();
        self.shop_button.handle_input();
    }

    pub fn sync_context(&self, context: &mut Context) {
        context.audio_on = self.audio_button.on;
        context.camera_locked = self.camera_button.on;
        context.settings_open = self.settings_button.on;
        context.shop_open = self.shop_button.on;
    }
}

impl Ui {
    pub async fn new(world: Rc<RefCell<World>>) -> Self {
        let icon_factory = IconFactory::new().await;
        let button_factory = ButtonFactory::new().await;
        let banner_factory: BannerFactory = BannerFactory::new().await;

        Self {
            map: Map::new(world),
            audio_button: button_factory.new_audio_button(&icon_factory),
            camera_button: button_factory.new_camera_button(&icon_factory),
            settings_button: button_factory.new_settings_button(&icon_factory),
            shop_button: button_factory.new_shop_button(&icon_factory),
            banner: banner_factory.new_horizzontal_banner(Vec2::new(0.0, 0.0)),
        }
    }
}

impl Drawable for Ui {
    fn draw(&mut self, context: &Context) {
        self.audio_button.draw(context);
        self.camera_button.draw(context);
        self.settings_button.draw(context);
        self.shop_button.draw(context);
        self.banner.draw(context);
    }
}
