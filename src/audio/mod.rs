use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use oxagaudiotool::{sound_config::OxAgSoundConfig, OxAgAudioTool};
use robotics_lib::{
    event::events::Event,
    world::{environmental_conditions::WeatherType, tile::TileType},
};

use crate::core::{context::Context, TICK_DURATION_DEFAULT, TICK_DURATION_MIN};

pub struct Audio {
    tool: OxAgAudioTool,
    weather_update: Instant,
}

impl Audio {
    pub fn new() -> Self {
        Self {
            tool: OxAgAudioTool::new(HashMap::default(), HashMap::default(), HashMap::default())
                .unwrap(),
            weather_update: Instant::now() - Duration::from_millis(10000),
        }
    }

    pub fn play_weather_music(&mut self, context: &Context, weather_type: &WeatherType) {
        if !context.audio_on {
            return;
        }

        if self.weather_update.elapsed() >= Duration::from_millis(9000) {
            match weather_type {
                WeatherType::Foggy => {
                    let _ = self.tool.play_audio(&OxAgSoundConfig::new_with_volume(
                        "assets/audio/weather/foggy.ogg",
                        context.volume_amplitude,
                    ));
                }
                WeatherType::Rainy => {
                    let _ = self.tool.play_audio(&OxAgSoundConfig::new_with_volume(
                        "assets/audio/weather/rainy.ogg",
                        context.volume_amplitude,
                    ));
                }
                WeatherType::Sunny => {
                    let _ = self.tool.play_audio(&OxAgSoundConfig::new_with_volume(
                        "assets/audio/weather/sunny.ogg",
                        context.volume_amplitude,
                    ));
                }
                WeatherType::TropicalMonsoon => {
                    let _ = self.tool.play_audio(&OxAgSoundConfig::new_with_volume(
                        "assets/audio/weather/tropical.ogg",
                        context.volume_amplitude,
                    ));
                }
                WeatherType::TrentinoSnow => {
                    let _ = self.tool.play_audio(&OxAgSoundConfig::new_with_volume(
                        "assets/audio/weather/snowy.ogg",
                        context.volume_amplitude,
                    ));
                }
            }

            self.weather_update = Instant::now();
        }
    }

    pub fn play_event_sound(&mut self, context: &Context, event: &Event) {
        if !context.audio_on {
            return;
        }

        match event {
            Event::Moved(tile, _) => {
                self.play_tile_sound(context, &tile.tile_type);
            }
            _ => {}
        }
    }

    fn play_tile_sound(&mut self, context: &Context, tile_type: &TileType) {
        match context.tick_duration {
            TICK_DURATION_MIN => match tile_type {
                TileType::Grass => {
                    let _ = self.tool.play_audio(&OxAgSoundConfig::new_with_volume(
                        "assets/audio/tile/500/grass.ogg",
                        context.volume_amplitude,
                    ));
                }
                TileType::Hill => {
                    let _ = self.tool.play_audio(&OxAgSoundConfig::new_with_volume(
                        "assets/audio/tile/500/hill.ogg",
                        context.volume_amplitude,
                    ));
                }
                TileType::Mountain => {
                    let _ = self.tool.play_audio(&OxAgSoundConfig::new_with_volume(
                        "assets/audio/tile/500/mountain.ogg",
                        context.volume_amplitude,
                    ));
                }
                TileType::Sand => {
                    let _ = self.tool.play_audio(&OxAgSoundConfig::new_with_volume(
                        "assets/audio/tile/500/sand.ogg",
                        context.volume_amplitude,
                    ));
                }
                TileType::Snow => {
                    let _ = self.tool.play_audio(&OxAgSoundConfig::new_with_volume(
                        "assets/audio/tile/500/snow.ogg",
                        context.volume_amplitude,
                    ));
                }
                TileType::Street => {
                    let _ = self.tool.play_audio(&OxAgSoundConfig::new_with_volume(
                        "assets/audio/tile/500/street.ogg",
                        context.volume_amplitude,
                    ));
                }
                TileType::ShallowWater => {
                    let _ = self.tool.play_audio(&OxAgSoundConfig::new_with_volume(
                        "assets/audio/tile/500/water.ogg",
                        context.volume_amplitude,
                    ));
                }
                _ => {}
            },
            TICK_DURATION_DEFAULT => match tile_type {
                TileType::Grass => {
                    let _ = self.tool.play_audio(&OxAgSoundConfig::new_with_volume(
                        "assets/audio/tile/1000/grass.ogg",
                        context.volume_amplitude,
                    ));
                }
                TileType::Hill => {
                    let _ = self.tool.play_audio(&OxAgSoundConfig::new_with_volume(
                        "assets/audio/tile/1000/hill.ogg",
                        context.volume_amplitude,
                    ));
                }
                TileType::Mountain => {
                    let _ = self.tool.play_audio(&OxAgSoundConfig::new_with_volume(
                        "assets/audio/tile/1000/mountain.ogg",
                        context.volume_amplitude,
                    ));
                }
                TileType::Sand => {
                    let _ = self.tool.play_audio(&OxAgSoundConfig::new_with_volume(
                        "assets/audio/tile/1000/sand.ogg",
                        context.volume_amplitude,
                    ));
                }
                TileType::Snow => {
                    let _ = self.tool.play_audio(&OxAgSoundConfig::new_with_volume(
                        "assets/audio/tile/1000/snow.ogg",
                        context.volume_amplitude,
                    ));
                }
                TileType::Street => {
                    let _ = self.tool.play_audio(&OxAgSoundConfig::new_with_volume(
                        "assets/audio/tile/1000/street.ogg",
                        context.volume_amplitude,
                    ));
                }
                TileType::ShallowWater => {
                    let _ = self.tool.play_audio(&OxAgSoundConfig::new_with_volume(
                        "assets/audio/tile/1000/water.ogg",
                        context.volume_amplitude,
                    ));
                }
                _ => {}
            },
            _ => match tile_type {
                TileType::Grass => {
                    let _ = self.tool.play_audio(&OxAgSoundConfig::new_with_volume(
                        "assets/audio/tile/1500/grass.ogg",
                        context.volume_amplitude,
                    ));
                }
                TileType::Hill => {
                    let _ = self.tool.play_audio(&OxAgSoundConfig::new_with_volume(
                        "assets/audio/tile/1500/hill.ogg",
                        context.volume_amplitude,
                    ));
                }
                TileType::Mountain => {
                    let _ = self.tool.play_audio(&OxAgSoundConfig::new_with_volume(
                        "assets/audio/tile/1500/mountain.ogg",
                        context.volume_amplitude,
                    ));
                }
                TileType::Sand => {
                    let _ = self.tool.play_audio(&OxAgSoundConfig::new_with_volume(
                        "assets/audio/tile/1500/sand.ogg",
                        context.volume_amplitude,
                    ));
                }
                TileType::Snow => {
                    let _ = self.tool.play_audio(&OxAgSoundConfig::new_with_volume(
                        "assets/audio/tile/1500/snow.ogg",
                        context.volume_amplitude,
                    ));
                }
                TileType::Street => {
                    let _ = self.tool.play_audio(&OxAgSoundConfig::new_with_volume(
                        "assets/audio/tile/1500/street.ogg",
                        context.volume_amplitude,
                    ));
                }
                TileType::ShallowWater => {
                    let _ = self.tool.play_audio(&OxAgSoundConfig::new_with_volume(
                        "assets/audio/tile/1500/water.ogg",
                        context.volume_amplitude,
                    ));
                }
                _ => {}
            },
        }
    }
}
