use std::collections::HashMap;

use oxagaudiotool::{sound_config::OxAgSoundConfig, OxAgAudioTool};
use robotics_lib::{
    event::events::Event,
    world::{
        environmental_conditions::WeatherType,
        tile::{Content, TileType},
    },
};

pub struct Audio {
    tool: OxAgAudioTool,
}

impl Audio {
    pub fn new() -> Self {
        let background_music =
            OxAgSoundConfig::new_looped_with_volume("data/audio/default/music.ogg", 2.0);

        let mut event = HashMap::new();
        // event.insert(Event::AddedToBackpack(Content::None, 0), OxAgSoundConfig::new("data/audio/default/event/event_add_to_backpack.ogg"));
        // event.insert(Event::RemovedFromBackpack(Content::None, 0), OxAgSoundConfig::new("data/audio/default/event/event_remove_from_backpack.ogg"));

        let mut tile_type = HashMap::new();
        // tile_type.insert(TileType::DeepWater, OxAgSoundConfig::new("data/audio/default/tile/tile_water.ogg"));
        // tile_type.insert(TileType::ShallowWater, OxAgSoundConfig::new("data/audio/default/tile/tile_water.ogg"));
        // tile_type.insert(TileType::Sand, OxAgSoundConfig::new("data/audio/default/tile/tile_sand.ogg"));
        // tile_type.insert(TileType::Grass, OxAgSoundConfig::new("data/audio/default/tile/tile_grass.ogg"));
        // tile_type.insert(TileType::Hill, OxAgSoundConfig::new("data/audio/default/tile/tile_grass.ogg"));
        // tile_type.insert(TileType::Mountain, OxAgSoundConfig::new("data/audio/default/tile/tile_mountain.ogg"));
        // tile_type.insert(TileType::Snow, OxAgSoundConfig::new("data/audio/default/tile/tile_snow.ogg"));
        // tile_type.insert(TileType::Lava, OxAgSoundConfig::new("data/audio/default/tile/tile_lava.ogg"));
        // tile_type.insert(TileType::Teleport(false), OxAgSoundConfig::new("data/audio/default/tile/tile_teleport.ogg"));
        // tile_type.insert(TileType::Street, OxAgSoundConfig::new("data/audio/default/tile/tile_street.ogg"));

        let mut weather = HashMap::new();
        // weather.insert(WeatherType::Rainy, OxAgSoundConfig::new("data/audio/default/weather/weather_rainy.ogg"));
        // weather.insert(WeatherType::Foggy, OxAgSoundConfig::new("data/audio/default/weather/weather_foggy.ogg"));
        // weather.insert(WeatherType::Sunny, OxAgSoundConfig::new("data/audio/default/weather/weather_sunny.ogg"));
        // weather.insert(WeatherType::TrentinoSnow, OxAgSoundConfig::new("data/audio/default/weather/weather_winter.ogg"));
        // weather.insert(WeatherType::TropicalMonsoon, OxAgSoundConfig::new("data/audio/default/weather/weather_tropical.ogg"));

        Self {
            tool: OxAgAudioTool::new(event, tile_type, weather).unwrap(),
        }
    }

    pub fn play_event(&mut self, event: &Event) {
        let _ = self.tool.play_audio_based_on_event(event);
    }
}
