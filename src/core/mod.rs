use bevy::prelude::*;

pub mod config;
pub mod save;
pub mod state;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::srgb(0.5, 0.8, 0.9)))
            .init_state::<state::GameState>()
            .add_systems(Startup, (setup_camera, save::load_save_data))
            .add_systems(Update, save::save_save_data);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
