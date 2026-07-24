use crate::core::config::GameConfig;
use crate::game::effects::ParticleEffects;
use bevy::prelude::*;
use bevy_hanabi::prelude::*;
use rand::{Rng, rng, rngs::ThreadRng};

#[derive(Component)]
pub struct Obstacle {
    pub pipe_direction: f32,
    pub scored: bool,
}

#[derive(Resource)]
pub struct EnvironmentData {
    pub window_dimensions: Vec2,
}

pub fn update_obstacles(
    time: Res<Time<Fixed>>,
    env_data: Res<EnvironmentData>,
    config: Res<GameConfig>,
    mut obstacle_query: Query<(&mut Obstacle, &mut Transform)>,
    mut score: ResMut<crate::game::Score>,
    mut save_data: ResMut<crate::core::save::SaveData>,
    effects: Option<Res<ParticleEffects>>,
    mut commands: Commands,
) {
    let mut rand = rng();
    let y_offset = generate_offset(&mut rand, &config);
    // Difficulty curve: speed increases with score, capped at max
    let current_speed = (config.obstacle_scroll_speed
        + config.speed_increase_per_score * score.0 as f32)
        .min(config.max_obstacle_scroll_speed);

    for (mut obstacle, mut transform) in obstacle_query.iter_mut() {
        transform.translation.x -= time.delta_secs() * current_speed;

        // Scoring logic (if passing center where bird is at x=0)
        if !obstacle.scored && transform.translation.x < 0.0 && obstacle.pipe_direction == 1. {
            obstacle.scored = true;
            score.0 += 1;
            if score.0 > save_data.high_score {
                save_data.high_score = score.0;
            }
            // Spawn sparkle at the pipe gap center
            if let Some(fx) = &effects {
                commands.spawn((
                    ParticleEffect::new(fx.pipe_sparkle.clone()),
                    Transform::from_translation(Vec3::new(
                        transform.translation.x,
                        transform.translation.y,
                        1.0,
                    )),
                ));
            }
        }

        // Loop pipes
        if transform.translation.x + config.obstacle_width * config.pixel_ratio / 2.
            < -env_data.window_dimensions.x / 2.
        {
            transform.translation.x +=
                config.obstacle_amount as f32 * config.obstacle_spacing * config.pixel_ratio;
            transform.translation.y =
                get_centered_pipe_position(&config) * obstacle.pipe_direction + y_offset;
            obstacle.scored = false; // Reset for next loop
        }
    }
}

pub fn get_centered_pipe_position(config: &GameConfig) -> f32 {
    (config.obstacle_height / 2. + config.obstacle_gap_size) * config.pixel_ratio
}

pub fn spawn_obstacles(
    commands: &mut Commands,
    rand: &mut ThreadRng,
    window_width: f32,
    pipe_image: &Handle<Image>,
    config: &GameConfig,
) {
    for i in 0..config.obstacle_amount {
        let y_offset = generate_offset(rand, config);
        let x_pos = window_width / 2. + (config.obstacle_spacing * config.pixel_ratio * i as f32);
        spawn_obstacle(
            Vec3::X * x_pos + Vec3::Y * (get_centered_pipe_position(config) + y_offset),
            1.,
            commands,
            pipe_image,
            config,
        );

        spawn_obstacle(
            Vec3::X * x_pos + Vec3::Y * (-get_centered_pipe_position(config) + y_offset),
            -1.,
            commands,
            pipe_image,
            config,
        );
    }
}

fn spawn_obstacle(
    translation: Vec3,
    //bottom or top of screen
    pipe_direction: f32,
    commands: &mut Commands,
    pipe_image: &Handle<Image>,
    config: &GameConfig,
) {
    commands.spawn((
        Sprite { image: pipe_image.clone(), ..Default::default() },
        Transform::from_translation(translation).with_scale(Vec3::new(
            config.pixel_ratio,
            config.pixel_ratio * -pipe_direction,
            config.pixel_ratio,
        )),
        Obstacle { pipe_direction, scored: false },
        crate::game::GameEntity,
    ));
}

fn generate_offset(rand: &mut ThreadRng, config: &GameConfig) -> f32 {
    rand.random_range(-config.obstacle_vertical_offset..config.obstacle_vertical_offset)
        * config.pixel_ratio
}
