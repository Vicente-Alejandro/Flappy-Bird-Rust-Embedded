use crate::core::config::GameConfig;
use crate::core::state::GameState;
use crate::game::player::Bird;
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
            .add_systems(Update, save::save_save_data)
            .add_systems(Update, camera_lead.run_if(in_state(GameState::Playing)));
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

/// Smoothly offsets the camera ahead of the bird's movement direction.
/// Gives the player more visual runway to react to incoming obstacles.
fn camera_lead(
    bird_query: Query<&Bird>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    config: Res<GameConfig>,
    time: Res<Time>,
) {
    let Ok(bird) = bird_query.single() else {
        return;
    };
    let Ok(mut cam_transform) = camera_query.single_mut() else {
        return;
    };

    // Map bird velocity to a horizontal lead offset:
    // Positive velocity (rising) → look slightly right; falling → neutral.
    let lead_strength = config.camera_lead_strength;
    let target_x = (bird.velocity / config.flap_force).clamp(-0.5, 1.0) * lead_strength;
    let target_y = 0.0_f32;

    // Smooth interpolation — low lerp factor keeps motion very subtle
    cam_transform.translation.x =
        cam_transform.translation.x.lerp(target_x, time.delta_secs() * 1.5);
    cam_transform.translation.y =
        cam_transform.translation.y.lerp(target_y, time.delta_secs() * 1.2);
}
