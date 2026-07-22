use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // Game mechanics (bird, obstacles, collision) will be registered here
    }
}
