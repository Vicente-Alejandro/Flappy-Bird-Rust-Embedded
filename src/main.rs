use bevy::prelude::*;
use bevy::window::{MonitorSelection, WindowMode};
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
        .run();
}
