use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
#[allow(dead_code)]
pub enum GameState {
    #[default]
    MainMenu,
    Playing,
    Paused,
    GameOver,
}
