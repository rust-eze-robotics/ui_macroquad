use macroquad::{
    camera::{self, Camera2D},
    experimental::camera::mouse::Camera,
    math::Vec2,
    texture::Texture2D,
    window::{screen_height, screen_width},
};

pub trait Drawable {
    fn draw(&mut self, camera: &Camera2D);
}
pub fn is_in_window(camera: &Camera2D, pos: &Vec2, offset: &Vec2, width: f32, height: f32) -> bool {
    let center = Vec2::new(
        pos.x - offset.x + (width * camera.zoom.x / 2.0),
        pos.y + offset.y + (height * camera.zoom.y / 2.0),
    );

    if f32::abs(camera.target.x - camera.offset.x - center.x)
        > (screen_width() + width) / camera.zoom.x / 2.0
    {
        return false;
    }

    if f32::abs(camera.target.y + camera.offset.y - center.y) - height / 2.0
        > (screen_height() + height) / camera.zoom.y / 2.0
    {
        return false;
    }

    print!("#");

    true
}
