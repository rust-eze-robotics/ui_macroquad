use macroquad::prelude::*;

use self::context::Context;

pub mod context;
pub mod events;

pub const ZOOM_MIN: f32 = 0.001;
pub const ZOOM_MAX: f32 = 0.0034;
pub const ZOOM_DEFAULT: f32 = 0.0015;

pub trait Drawable {
    fn draw(&mut self, context: &Context);
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AnchorPosition {
    TopLeft(Vec2),
    TopRight(Vec2),
    DownLeft(Vec2),
    DownRight(Vec2),
}

pub fn get_current_anchor_position(anchor_pos: AnchorPosition) -> Vec2 {
    match anchor_pos {
        AnchorPosition::TopLeft(v) => Vec2::new(0.0, 0.0) + v,
        AnchorPosition::TopRight(v) => Vec2::new(screen_width(), 0.0) + v,
        AnchorPosition::DownLeft(v) => Vec2::new(0.0, screen_height()) + v,
        AnchorPosition::DownRight(v) => Vec2::new(screen_width(), screen_height()) + v,
    }
}

pub fn is_hovered(pos: &Vec2, size: &Vec2) -> bool {
    let mouse = mouse_position();

    if (mouse.0 >= pos.x && mouse.0 <= pos.x + size.x)
        && (mouse.1 >= pos.y && mouse.1 <= pos.y + size.x)
    {
        true
    } else {
        false
    }
}

pub fn is_down(pos: &Vec2, size: &Vec2) -> bool {
    if is_mouse_button_down(MouseButton::Left) && is_hovered(pos, size) {
        true
    } else {
        false
    }
}

pub fn is_pressed(pos: &Vec2, size: &Vec2) -> bool {
    if is_mouse_button_pressed(MouseButton::Left) && is_hovered(pos, size) {
        true
    } else {
        false
    }
}

pub fn is_released(pos: &Vec2, size: &Vec2) -> bool {
    if is_mouse_button_released(MouseButton::Left) && is_hovered(pos, size) {
        true
    } else {
        false
    }
}

pub fn is_in_window(context: &Context, pos: &Vec2, offset: &Vec2, width: f32, height: f32) -> bool {
    let left_up_corner = context
        .camera
        .world_to_screen(Vec2::new(pos.x - offset.x, pos.y + offset.y));

    let right_down_corner = context.camera.world_to_screen(Vec2::new(
        pos.x - offset.x + width,
        pos.y + offset.y + height,
    ));

    if (right_down_corner.x <= 0.0) || (left_up_corner.x >= screen_width()) {
        return false;
    }

    if (right_down_corner.y <= 0.0) || (left_up_corner.y >= screen_height()) {
        return false;
    }

    true
}
