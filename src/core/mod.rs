use macroquad::prelude::*;

pub trait Drawable {
    fn draw(&mut self, camera: &Camera2D);
}
pub fn is_in_window(camera: &Camera2D, pos: &Vec2, offset: &Vec2, width: f32, height: f32) -> bool {
    let left_up_corner = camera.world_to_screen(Vec2::new(pos.x - offset.x, pos.y + offset.y));

    let right_down_corner = camera.world_to_screen(Vec2::new(
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
