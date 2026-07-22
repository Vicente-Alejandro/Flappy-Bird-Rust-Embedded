use bevy::prelude::*;

pub struct HardwarePlugin;

impl Plugin for HardwarePlugin {
    fn build(&self, _app: &mut App) {
        // Hardware integration (schematics rendering, serial IO, telemetry export)
    }
}
