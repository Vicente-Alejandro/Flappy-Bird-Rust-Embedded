use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Resource, Serialize, Deserialize, Debug, Clone, Default)]
pub struct SaveData {
    pub high_score: u32,
}

const SAVE_PATH: &str = "save.ron";

pub fn load_save_data(mut commands: Commands) {
    let save_data = if let Ok(data) = fs::read_to_string(SAVE_PATH) {
        ron::from_str(&data).unwrap_or_default()
    } else {
        SaveData::default()
    };
    commands.insert_resource(save_data);
}

pub fn save_save_data(save_data: Res<SaveData>) {
    if save_data.is_changed() {
        let Ok(data) = ron::ser::to_string_pretty(&*save_data, ron::ser::PrettyConfig::default())
        else {
            return;
        };
        let _ = fs::write(SAVE_PATH, data);
    }
}
