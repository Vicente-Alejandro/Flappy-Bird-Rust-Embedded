use crate::core::config::GameConfig;
use bevy::prelude::*;

#[derive(Component)]
pub struct Bird {
    pub velocity: f32,
    pub jump_intent: bool,
}

#[derive(Component)]
pub struct ProceduralAnimation {
    pub scale_target: Vec2,
    pub flap_timer: Timer,
    pub frames: [Handle<Image>; 4],
    pub current_frame: usize,
}

pub fn handle_jump_input(
    mut bird_query: Query<(&mut Bird, &mut ProceduralAnimation)>,
    keys: Res<ButtonInput<KeyCode>>,
    config: Res<GameConfig>,
) {
    if let Ok((mut bird, mut anim)) = bird_query.single_mut() {
        if keys.just_pressed(KeyCode::Space) {
            bird.jump_intent = true;
            // Stretch vertically, squash horizontally
            let base = config.bird_scale * config.pixel_ratio;
            anim.scale_target = Vec2::new(0.7 * base, 1.4 * base);
        }
    }
}

pub fn update_bird_movement(
    mut bird_query: Query<(&mut Bird, &mut Transform)>,
    time: Res<Time<Fixed>>,
    config: Res<GameConfig>,
) {
    if let Ok((mut bird, mut transform)) = bird_query.single_mut() {
        if bird.jump_intent {
            bird.velocity = config.flap_force;
            bird.jump_intent = false;
        }

        bird.velocity -= time.delta_secs() * config.gravity;
        transform.translation.y += bird.velocity * time.delta_secs();
    }
}

pub fn update_procedural_animation(
    mut query: Query<(&mut Transform, &mut Sprite, &mut ProceduralAnimation, &Bird)>,
    time: Res<Time>,
    config: Res<GameConfig>,
) {
    for (mut transform, mut sprite, mut anim, bird) in query.iter_mut() {
        // 1. Squash & Stretch (Lerp back to normal scale)
        let base = config.bird_scale * config.pixel_ratio;
        anim.scale_target = anim.scale_target.lerp(Vec2::splat(base), time.delta_secs() * 10.0);
        transform.scale = anim.scale_target.extend(1.0); // Z=1.0 is scale identity

        // 2. Velocity Rotation
        let target_angle =
            f32::clamp(bird.velocity / config.velocity_to_rotation_ratio, -90., 90.).to_radians();
        let target_quat = Quat::from_axis_angle(Vec3::Z, target_angle);
        transform.rotation = transform.rotation.slerp(target_quat, time.delta_secs() * 15.0);

        // 3. Wing Flapping
        anim.flap_timer.tick(time.delta());
        if anim.flap_timer.just_finished() {
            anim.current_frame = (anim.current_frame + 1) % anim.frames.len();
            sprite.image = anim.frames[anim.current_frame].clone();
        }
    }
}
