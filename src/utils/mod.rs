use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub esp_enabled: bool,
    pub item_esp: bool,
    pub aimbot_enabled: bool,
    pub triggerbot: bool,
    pub max_distance: f32,
    pub fov: f32,
}

impl Config {
    pub fn load() -> Self {
        fs::read_to_string("config.json")
            .map(|s| serde_json::from_str(&s).unwrap())
            .unwrap_or_else(|_| Self::default())
    }

    pub fn save(&self) {
        let _ = fs::write("config.json", serde_json::to_string_pretty(self).unwrap());
    }

    fn default() -> Self {
        Self {
            esp_enabled: false,
            item_esp: false,
            aimbot_enabled: false,
            triggerbot: false,
            max_distance: 300.0,
            fov: 90.0,
        }
    }
}