use std::{cell::RefCell, rc::Rc, time::Duration};

use crate::{
    core::{context::Context, Drawable, TICK_DURATION_MAX},
    world::World,
};

use self::{
    component::{
        button::{factory::ButtonFactory, Button},
        clicker::factory::ClickerFactory,
        icon::factory::IconFactory,
        stepper::factory::StepperFactory,
    },
    settings::SettingsModal,
};

pub mod component;
pub mod settings;
pub mod shop;

pub trait UiItem: Drawable {
    fn update_gui(&mut self, context: &Context);
    fn handle_input(&mut self, context: &Context);
}

pub struct Ui {
    audio_button: Button,
    camera_button: Button,
    settings_button: Button,
    shop_button: Button,
    settings_modal: SettingsModal,
}

impl Ui {
    pub fn update_gui(&mut self, context: &Context) {
        self.audio_button.update_gui(context);
        self.camera_button.update_gui(context);
        self.settings_button.update_gui(context);
        self.shop_button.update_gui(context);
        self.settings_modal.update_gui(context);
    }

    pub fn handle_input(&mut self, context: &Context) {
        self.audio_button.handle_input(context);
        self.camera_button.handle_input(context);
        self.settings_button.handle_input(context);
        self.shop_button.handle_input(context);
        self.settings_modal.handle_input(context);
    }

    pub fn sync_context(&self, context: &mut Context) {
        context.tick_duration = Duration::from_millis(self.settings_modal.get_speed() as u64);
        context.volume_amplitude = self.settings_modal.get_volume() as f64 / 50.0;
        context.audio_on = self.audio_button.on;
        context.camera_locked = self.camera_button.on;
        context.settings_open = !self.settings_button.on;
        context.shop_open = !self.shop_button.on;
    }
}

impl Ui {
    pub async fn new(_world: Rc<RefCell<World>>) -> Self {
        let icon_factory = IconFactory::new().await;
        let button_factory = ButtonFactory::new().await;
        let clicker_factory = ClickerFactory::new().await;
        let stepper_factory = StepperFactory::new().await;

        Self {
            audio_button: button_factory.new_audio_button(&icon_factory),
            camera_button: button_factory.new_camera_button(&icon_factory),
            settings_button: button_factory.new_settings_button(&icon_factory),
            shop_button: button_factory.new_shop_button(&icon_factory),
            settings_modal: SettingsModal::new(&icon_factory, &clicker_factory, &stepper_factory)
                .await,
        }
    }
}

impl Drawable for Ui {
    fn draw(&mut self, context: &Context) {
        self.audio_button.draw(context);
        self.camera_button.draw(context);
        self.settings_button.draw(context);
        self.shop_button.draw(context);
        self.settings_modal.draw(context);
    }
}
