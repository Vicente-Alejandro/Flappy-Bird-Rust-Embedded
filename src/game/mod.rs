use crate::core::config::GameConfig;
use crate::core::state::GameState;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::rng;

pub mod collision;
pub mod environment;
pub mod player;

#[derive(Resource, Default)]
pub struct Score(pub u32);

#[derive(Component)]
pub struct GameEntity; // Marker component to despawn everything when game over

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .add_systems(OnEnter(GameState::Playing), setup_game)
            .add_systems(OnExit(GameState::Playing), cleanup_game)
            .add_systems(
                FixedUpdate,
                (
                    player::update_bird_movement,
                    environment::update_obstacles,
                    collision::check_collisions,
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    config: Res<GameConfig>,
    mut score: ResMut<Score>,
) {
    score.0 = 0; // Reset score

    let pipe_image = asset_server.load("pipe.png");
    let Ok(window) = window_query.single() else {
        return;
    };

    commands.insert_resource(environment::EnvironmentData {
        window_dimensions: Vec2::new(window.width(), window.height()),
    });

    commands.spawn((
        Sprite { image: asset_server.load("bird.png"), ..Default::default() },
        Transform::IDENTITY.with_scale(Vec3::splat(config.pixel_ratio)),
        player::Bird { velocity: 0. },
        GameEntity,
    ));

    let mut rand = rng();
    environment::spawn_obstacles(&mut commands, &mut rand, 1080., &pipe_image, &config);
}

fn cleanup_game(mut commands: Commands, query: Query<Entity, With<GameEntity>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
