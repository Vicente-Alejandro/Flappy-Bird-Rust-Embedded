use crate::core::config::GameConfig;
use crate::core::state::GameState;
use crate::game::environment::EnvironmentData;
use crate::game::environment::Obstacle;
use crate::game::player::Bird;
use bevy::prelude::*;

pub fn check_collisions(
    bird_query: Query<&Transform, With<Bird>>,
    obstacle_query: Query<&Transform, With<Obstacle>>,
    env_data: Res<EnvironmentData>,
    config: Res<GameConfig>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if let Ok(bird_transform) = bird_query.single() {
        let mut dead = false;

        // Floor/Ceiling collision
        if bird_transform.translation.y <= -env_data.window_dimensions.y / 2.
            || bird_transform.translation.y >= env_data.window_dimensions.y / 2.
        {
            dead = true;
        } else {
            // Pipe collision
            for pipe_transform in obstacle_query.iter() {
                if (pipe_transform.translation.y - bird_transform.translation.y).abs()
                    < config.obstacle_height * config.pixel_ratio / 2.
                    && (pipe_transform.translation.x - bird_transform.translation.x).abs()
                        < config.obstacle_width * config.pixel_ratio / 2.
                {
                    dead = true;
                    break;
                }
            }
        }

        if dead {
            next_state.set(GameState::GameOver);
        }
    }
}
