use macroquad::texture::Texture2D;

pub trait Drawable {
    fn draw(&mut self);
}
