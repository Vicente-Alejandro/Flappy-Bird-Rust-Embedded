use bevy::prelude::*;

pub mod config;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, _app: &mut App) {
        // Core features (state, config, telemetry) will be registered here
    }
}
