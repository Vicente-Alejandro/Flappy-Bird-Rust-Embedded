use crate::core::config::GameConfig;
use bevy::prelude::*;

#[derive(Component)]
pub struct Bird {
    pub velocity: f32,
}

pub fn update_bird_movement(
    mut bird_query: Query<(&mut Bird, &mut Transform)>,
    time: Res<Time<Fixed>>,
    keys: Res<ButtonInput<KeyCode>>,
    config: Res<GameConfig>,
) {
    if let Ok((mut bird, mut transform)) = bird_query.single_mut() {
        let jumped = keys.just_pressed(KeyCode::Space);

        if jumped {
            bird.velocity = config.flap_force;
        }

        bird.velocity -= time.delta_secs() * config.gravity;
        transform.translation.y += bird.velocity * time.delta_secs();

        transform.rotation = Quat::from_axis_angle(
            Vec3::Z,
            f32::clamp(bird.velocity / config.velocity_to_rotation_ratio, -90., 90.).to_radians(),
        );
    }
}
