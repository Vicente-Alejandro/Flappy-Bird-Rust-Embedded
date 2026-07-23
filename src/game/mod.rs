use crate::core::config::GameConfig;
use crate::core::state::GameState;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::{Rng, rng};

pub mod collision;
pub mod environment;
pub mod player;

#[derive(Resource, Default)]
pub struct Score(pub u32);

#[derive(Resource, Default)]
pub struct Paused(pub bool);

#[derive(Component)]
pub struct GameEntity; // Marker component to despawn everything when game over

fn toggle_pause(keys: Res<ButtonInput<KeyCode>>, mut paused: ResMut<Paused>) {
    if keys.just_pressed(KeyCode::Escape) || keys.just_pressed(KeyCode::KeyP) {
        paused.0 = !paused.0;
    }
}

fn is_playing_and_unpaused(state: Res<State<GameState>>, paused: Res<Paused>) -> bool {
    *state.get() == GameState::Playing && !paused.0
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .init_resource::<Paused>()
            .add_systems(OnEnter(GameState::Playing), setup_game)
            .add_systems(OnExit(GameState::Playing), cleanup_game)
            .add_systems(
                Update,
                (player::handle_jump_input, player::update_procedural_animation, toggle_pause)
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                FixedUpdate,
                (
                    player::update_bird_movement,
                    environment::update_obstacles,
                    collision::check_collisions,
                )
                    .run_if(is_playing_and_unpaused),
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
    commands.insert_resource(Paused(false));

    let mut rand = rng();
    let colors = ["bluebird", "redbird", "yellowbird"];
    let selected_color = colors[rand.random_range(0..3)];
    let frame0 = asset_server.load(format!("bird_spritesheets/{}-downflap.png", selected_color));
    let frame1 = asset_server.load(format!("bird_spritesheets/{}-midflap.png", selected_color));
    let frame2 = asset_server.load(format!("bird_spritesheets/{}-upflap.png", selected_color));
    let frame3 = frame1.clone();

    let frames = [frame0.clone(), frame1, frame2, frame3];

    commands.spawn((
        Sprite { image: frame0, ..Default::default() },
        Transform::IDENTITY.with_scale(Vec3::splat(config.bird_scale * config.pixel_ratio)),
        player::Bird { velocity: 0., jump_intent: false },
        player::ProceduralAnimation {
            scale_target: Vec2::splat(config.bird_scale * config.pixel_ratio),
            flap_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            frames,
            current_frame: 0,
        },
        GameEntity,
    ));

    environment::spawn_obstacles(&mut commands, &mut rand, 1080., &pipe_image, &config);
}

fn cleanup_game(mut commands: Commands, query: Query<Entity, With<GameEntity>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
