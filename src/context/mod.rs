use std::time::Instant;

use macroquad::prelude::*;

pub const CLOCK_MS: u64 = 1000;
const ZOOM_MIN: f32 = 0.001;
const ZOOM_MAX: f32 = 0.0034;
const ZOOM_DEFAULT: f32 = 0.0015;

pub struct Context {
    pub timestamp: Instant,
    pub camera: Camera2D,
}

impl Context {
    pub fn new(target: Vec2, offset: Vec2) -> Self {
        Self {
            timestamp: Instant::now(),
            camera: Camera2D {
                target,
                offset,
                zoom: Vec2::new(ZOOM_DEFAULT, ZOOM_DEFAULT),
                ..Default::default()
            },
        }
    }

    pub fn camera(&self) -> Camera2D {
        Camera2D {
            target: self.camera.target,
            offset: self.camera.offset,
            zoom: self.camera.zoom,
            ..Default::default()
        }
    }

    pub fn update_camera(&mut self) {
        if is_key_down(KeyCode::Left) {
            self.camera.offset.x += 0.1;
        }

        if is_key_down(KeyCode::Right) {
            self.camera.offset.x -= 0.1;
        }

        if is_key_down(KeyCode::Up) {
            self.camera.offset.y -= 0.1;
        }

        if is_key_down(KeyCode::Down) {
            self.camera.offset.y += 0.1;
        }

        if is_key_down(KeyCode::Space) {
            self.camera.offset.x = 0.0;
            self.camera.offset.y = 0.0;
        }

        if mouse_wheel().1 != 0.0 {
            self.camera.zoom.x *= 1.5f32.powf(mouse_wheel().1);
            if !is_key_down(KeyCode::LeftControl) {
                self.camera.zoom.x = clamp(self.camera.zoom.x, ZOOM_MIN, ZOOM_MAX);
            }
            self.camera.zoom.y = self.camera.zoom.x * screen_width() / screen_height();
        }
    }
}
