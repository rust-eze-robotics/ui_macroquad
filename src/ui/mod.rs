use std::{cell::RefCell, rc::Rc, time::Duration};

use crate::{
    core::{context::Context, Drawable},
    robot::Robot,
    world::World,
};

use self::{
    bar::Bar,
    component::{
        button::{factory::ButtonFactory, Button},
        clicker::factory::ClickerFactory,
        icon::factory::IconFactory,
        stepper::factory::StepperFactory,
    },
    cursor::Cursor,
    settings::SettingsModal,
};

pub mod bar;
pub mod component;
pub mod cursor;
pub mod settings;
pub mod shop;

pub trait UiItem: Drawable {
    fn update_gui(&mut self);
    fn handle_input(&mut self);
}

pub struct Ui {
    pub energy_bar: Bar,
    cursor: Cursor,
    audio_button: Button,
    camera_button: Button,
    settings_button: Button,
    shop_button: Button,
    settings_modal: SettingsModal,
}

impl Ui {
    pub fn update_gui(&mut self, context: &Context) {
        self.energy_bar.update_gui();
        self.audio_button.update_gui();
        self.camera_button.update_gui();
        self.settings_button.update_gui();
        self.shop_button.update_gui();
        self.settings_modal.update_gui();
    }

    pub fn handle_input(&mut self, context: &Context) {
        self.energy_bar.handle_input();
        self.audio_button.handle_input();
        self.camera_button.handle_input();
        self.settings_button.handle_input();
        self.shop_button.handle_input();
        self.settings_modal.handle_input();
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
            cursor: Cursor::new().await,
            energy_bar: Bar::new().await,
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
        self.energy_bar.draw(context);
        self.audio_button.draw(context);
        self.camera_button.draw(context);
        self.settings_button.draw(context);
        self.shop_button.draw(context);
        self.settings_modal.draw(context);
        self.cursor.draw(context);
    }
}
