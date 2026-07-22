use bevy::window::{MonitorSelection, WindowMode};
use bevy::{prelude::*, window::PrimaryWindow};
use rand::{Rng, rng, rngs::ThreadRng};
use std::fs;

mod core;
mod game;
mod hardware;
mod ui;

fn main() {
    tracing_subscriber::fmt::init();
    info!("Starting Flappy Bird...");

    let config_str = fs::read_to_string("assets/config.ron")
        .expect("Failed to read assets/config.ron. Does the file exist?");
    let config: core::config::GameConfig =
        ron::from_str(&config_str).expect("Failed to parse config.ron. Is the syntax correct?");

    App::new()
        .insert_resource(config)
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Flappy Bird"),
                        mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        // Register domain plugins
        .add_plugins(core::CorePlugin)
        .add_plugins(game::GamePlugin)
        .add_plugins(ui::UiPlugin)
        .add_plugins(hardware::HardwarePlugin)
        // Legacy systems (to be refactored in v0.3.0)
        .add_systems(Startup, setup_level)
        .add_systems(FixedUpdate, (update_bird, update_obstacles))
        .run();
}

#[derive(Resource)]
pub struct GameManager {
    pub pipe_image: Handle<Image>,
    pub window_dimensions: Vec2,
}

#[derive(Component)]
struct Bird {
    pub velocity: f32,
}
fn setup_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&mut Window, With<PrimaryWindow>>,
    config: Res<core::config::GameConfig>,
) {
    let pipe_image = asset_server.load("pipe.png");
    let window = window_query.single().unwrap();
    commands.insert_resource(GameManager {
        pipe_image: pipe_image.clone(),
        window_dimensions: Vec2::new(window.width(), window.height()),
    });
    commands.insert_resource(ClearColor(Color::srgb(0.5, 0.7, 0.8)));

    commands.spawn(Camera2d);

    commands.spawn((
        Sprite { image: asset_server.load("bird.png"), ..Default::default() },
        Transform::IDENTITY.with_scale(Vec3::splat(config.pixel_ratio)),
        Bird { velocity: 0. },
    ));

    let mut rand = rng();

    spawn_obstacles(&mut commands, &mut rand, 1080., &pipe_image, &config);
}
#[derive(Component)]
struct Obstacle {
    pipe_direction: f32,
}
fn update_obstacles(
    time: Res<Time<Fixed>>,
    game_manager: Res<GameManager>,
    config: Res<core::config::GameConfig>,
    mut obstacle_query: Query<(&mut Obstacle, &mut Transform)>,
) {
    let mut rand = rng();
    let y_offset = generate_offset(&mut rand, &config);
    for (obstacle, mut transform) in obstacle_query.iter_mut() {
        transform.translation.x -= time.delta_secs() * config.obstacle_scroll_speed;

        if transform.translation.x + config.obstacle_width * config.pixel_ratio / 2.
            < -game_manager.window_dimensions.x / 2.
        {
            transform.translation.x +=
                config.obstacle_amount as f32 * config.obstacle_spacing * config.pixel_ratio;
            transform.translation.y =
                get_centered_pipe_position(&config) * obstacle.pipe_direction + y_offset;
        }
    }
}
fn get_centered_pipe_position(config: &core::config::GameConfig) -> f32 {
    (config.obstacle_height / 2. + config.obstacle_gap_size) * config.pixel_ratio
}
fn spawn_obstacles(
    commands: &mut Commands,
    rand: &mut ThreadRng,
    window_width: f32,
    pipe_image: &Handle<Image>,
    config: &core::config::GameConfig,
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
    config: &core::config::GameConfig,
) {
    commands.spawn((
        Sprite { image: pipe_image.clone(), ..Default::default() },
        Transform::from_translation(translation).with_scale(Vec3::new(
            config.pixel_ratio,
            config.pixel_ratio * -pipe_direction,
            config.pixel_ratio,
        )),
        Obstacle { pipe_direction },
    ));
}
fn generate_offset(rand: &mut ThreadRng, config: &core::config::GameConfig) -> f32 {
    rand.random_range(-config.obstacle_vertical_offset..config.obstacle_vertical_offset)
        * config.pixel_ratio
}
fn update_bird(
    mut commands: Commands,
    mut bird_query: Query<(&mut Bird, &mut Transform), Without<Obstacle>>,
    mut obstacle_query: Query<(&Transform, Entity), With<Obstacle>>,
    time: Res<Time<Fixed>>,
    keys: Res<ButtonInput<KeyCode>>,
    game_manager: Res<GameManager>,
    config: Res<core::config::GameConfig>,
) {
    if let Ok((mut bird, mut transform)) = bird_query.single_mut() {
        if keys.just_pressed(KeyCode::Space) {
            bird.velocity = config.flap_force;
        }

        bird.velocity -= time.delta_secs() * config.gravity;
        transform.translation.y += bird.velocity * time.delta_secs();

        transform.rotation = Quat::from_axis_angle(
            Vec3::Z,
            f32::clamp(bird.velocity / config.velocity_to_rotation_ratio, -90., 90.).to_radians(),
        );

        let mut dead = false;
        if transform.translation.y <= -game_manager.window_dimensions.y / 2. {
            dead = true;
        } else {
            for (pipe_transform, _entity) in obstacle_query.iter() {
                //collision check
                if (pipe_transform.translation.y - transform.translation.y).abs()
                    < config.obstacle_height * config.pixel_ratio / 2.
                    && (pipe_transform.translation.x - transform.translation.x).abs()
                        < config.obstacle_width * config.pixel_ratio / 2.
                {
                    dead = true;
                    break;
                }
            }
        }
        if dead {
            transform.translation = Vec3::ZERO;
            bird.velocity = 0.;
            for (_pipe_transform, entity) in obstacle_query.iter_mut() {
                commands.entity(entity).despawn();
            }
            let mut rand = rng();
            spawn_obstacles(
                &mut commands,
                &mut rand,
                game_manager.window_dimensions.x,
                &game_manager.pipe_image,
                &config,
            );
        }
    }
}
