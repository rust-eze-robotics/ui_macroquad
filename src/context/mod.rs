use std::time::Instant;

use macroquad::prelude::*;

use crate::core::{ZOOM_DEFAULT, ZOOM_MAX, ZOOM_MIN};

pub struct Context {
    pub timestamp: Instant,
    pub camera: Camera2D,
    pub audio_on: bool,
    pub camera_locked: bool,
    pub settings_open: bool,
    pub shop_open: bool,
}

impl Context {
    pub fn new(camera: Camera2D) -> Self {
        Self {
            timestamp: Instant::now(),
            camera,
            audio_on: true,
            camera_locked: true,
            settings_open: false,
            shop_open: false,
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

    pub fn update_camera(&mut self, target_pos: Vec2) {
        if self.camera_locked {
            self.camera.target = target_pos;
        }

        if is_key_down(KeyCode::Left) {
            self.camera.offset.x += 0.05;
        }

        if is_key_down(KeyCode::Right) {
            self.camera.offset.x -= 0.05;
        }

        if is_key_down(KeyCode::Up) {
            self.camera.offset.y -= 0.05;
        }

        if is_key_down(KeyCode::Down) {
            self.camera.offset.y += 0.05;
        }

        if is_key_down(KeyCode::Space) {
            self.camera.offset.x = 0.0;
            self.camera.offset.y = 0.0;
            self.camera.zoom.x = ZOOM_DEFAULT;
        }

        if mouse_wheel().1 != 0.0 {
            self.camera.zoom.x *= 1.5f32.powf(mouse_wheel().1);

            if !is_key_down(KeyCode::LeftControl) {
                self.camera.zoom.x = clamp(self.camera.zoom.x, ZOOM_MIN, ZOOM_MAX);
            }
        }

        self.camera.zoom.y = self.camera.zoom.x * screen_width() / screen_height();
    }
}
