use bevy::prelude::*;
use serde::Deserialize;

#[derive(Resource, Deserialize, Debug, Clone)]
pub struct GameConfig {
    pub pixel_ratio: f32,
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
}
