use std::rc::Rc;

use macroquad::{experimental::animation::AnimatedSprite, texture::Texture2D};

use crate::core::context::Context;

pub mod archer;
pub mod factory;
pub mod pawn;
pub mod torch;
pub mod warrior;

pub trait Character {
    fn get_texture(&self) -> Rc<Texture2D>;
    fn get_init_sprite(&self) -> AnimatedSprite;
    fn get_idle_sprite(&self, context: &Context) -> AnimatedSprite;
    fn get_interact_right_sprite(&self, context: &Context) -> AnimatedSprite;
    fn get_interact_up_sprite(&self, context: &Context) -> AnimatedSprite;
    fn get_interact_down_sprite(&self, context: &Context) -> AnimatedSprite;
}
