use std::{cell::RefCell, rc::Rc};

use macroquad::prelude::*;

use crate::{
    core::{context::Context, is_released, Drawable, SHOP_MODAL_SIZE},
    robot::{
        character::{self, factory::CharacterFactory, Character},
        Robot,
    },
};

use super::{
    component::selector::{factory::SelectorFactory, Selector},
    UiItem,
};

pub struct ShopModal {
    pub pos: Vec2,
    archer_selector: Selector,
    pawn_selector: Selector,
    torch_selector: Selector,
    warrior_selector: Selector,
}

impl ShopModal {
    pub fn new(selector_factory: &SelectorFactory) -> Self {
        let pos = Vec2::new(
            (screen_width() - SHOP_MODAL_SIZE.x) / 2.0,
            (screen_height() - SHOP_MODAL_SIZE.y) / 2.0,
        );

        Self {
            pos,
            archer_selector: selector_factory.new_archer_selector(),
            pawn_selector: selector_factory.new_pawn_selector(),
            torch_selector: selector_factory.new_torch_selector(),
            warrior_selector: selector_factory.new_warrior_selector(),
        }
    }

    pub fn update_character(&self, robot: &mut Robot) {
        if is_released(&self.archer_selector.pos, &self.archer_selector.size) {
            robot.set_archer();
        } else if is_released(&self.pawn_selector.pos, &self.pawn_selector.size) {
            robot.set_pawn();
        } else if is_released(&self.torch_selector.pos, &self.torch_selector.size) {
            robot.set_torch();
        } else if is_released(&self.warrior_selector.pos, &self.warrior_selector.size) {
            robot.set_warrior();
        }
    }
}

impl UiItem for ShopModal {
    fn update_gui(&mut self) {
        self.pos = Vec2::new(
            (screen_width() - SHOP_MODAL_SIZE.x) / 2.0,
            (screen_height() - SHOP_MODAL_SIZE.y) / 2.0,
        );

        self.warrior_selector.pos = self.pos;
        self.pawn_selector.pos =
            self.warrior_selector.pos + Vec2::new(self.warrior_selector.size.x, 0.0);
        self.archer_selector.pos =
            self.warrior_selector.pos + Vec2::new(0.0, self.warrior_selector.size.y);
        self.torch_selector.pos = self.warrior_selector.pos + self.warrior_selector.size;

        self.archer_selector.update_gui();
        self.pawn_selector.update_gui();
        self.torch_selector.update_gui();
        self.warrior_selector.update_gui();
    }

    fn handle_input(&mut self) {
        self.archer_selector.handle_input();
        self.pawn_selector.handle_input();
        self.torch_selector.handle_input();
        self.warrior_selector.handle_input();
    }
}

impl Drawable for ShopModal {
    fn draw(&mut self, context: &Context) {
        if context.shop_open {
            draw_rectangle(
                self.pos.x,
                self.pos.y,
                SHOP_MODAL_SIZE.x,
                SHOP_MODAL_SIZE.y,
                Color::from_rgba(190, 162, 136, 255),
            );

            draw_rectangle_lines(
                self.pos.x,
                self.pos.y,
                SHOP_MODAL_SIZE.x,
                SHOP_MODAL_SIZE.y,
                5.0,
                GOLD,
            );

            self.archer_selector.draw(context);
            self.pawn_selector.draw(context);
            self.torch_selector.draw(context);
            self.warrior_selector.draw(context);
        }
    }
}
