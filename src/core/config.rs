use bevy::prelude::*;
use serde::Deserialize;

#[derive(Resource, Deserialize, Debug, Clone)]
pub struct GameConfig {
    pub pixel_ratio: f32,
    #[serde(default = "default_bird_scale")]
    pub bird_scale: f32,
    pub flap_force: f32,
    pub gravity: f32,
    pub velocity_to_rotation_ratio: f32,
    pub obstacle_amount: i32,
    pub obstacle_width: f32,
    pub obstacle_height: f32,
    pub obstacle_vertical_offset: f32,
    pub obstacle_gap_size: f32,
    pub obstacle_spacing: f32,
    pub obstacle_scroll_speed: f32,
    #[serde(default = "default_camera_lead_strength")]
    pub camera_lead_strength: f32,
    /// Speed added per point scored (difficulty ramp).
    #[serde(default = "default_speed_increase_per_score")]
    pub speed_increase_per_score: f32,
    /// Hard cap on obstacle scroll speed regardless of score.
    #[serde(default = "default_max_obstacle_scroll_speed")]
    pub max_obstacle_scroll_speed: f32,
}

fn default_bird_scale() -> f32 {
    1.0
}

fn default_camera_lead_strength() -> f32 {
    60.0
}

fn default_speed_increase_per_score() -> f32 {
    2.0
}

fn default_max_obstacle_scroll_speed() -> f32 {
    350.0
}
