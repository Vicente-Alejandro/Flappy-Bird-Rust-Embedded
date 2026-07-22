use bevy::prelude::*;

pub mod config;
pub mod save;
pub mod state;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<state::GameState>()
            .add_systems(Startup, save::load_save_data)
            .add_systems(Update, save::save_save_data);
    }
}
