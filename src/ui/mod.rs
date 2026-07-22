use crate::core::save::SaveData;
use crate::core::state::GameState;
use crate::game::Score;
use bevy::prelude::*;

pub struct UiPlugin;

#[derive(Component)]
struct MenuUi;

#[derive(Component)]
struct ScoreText;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
            .add_systems(Update, menu_input.run_if(in_state(GameState::MainMenu)))
            .add_systems(OnExit(GameState::MainMenu), cleanup_menu)
            .add_systems(OnEnter(GameState::Playing), setup_hud)
            .add_systems(Update, update_hud.run_if(in_state(GameState::Playing)))
            .add_systems(OnExit(GameState::Playing), cleanup_hud)
            .add_systems(OnEnter(GameState::GameOver), setup_game_over)
            .add_systems(Update, game_over_input.run_if(in_state(GameState::GameOver)))
            .add_systems(OnExit(GameState::GameOver), cleanup_menu);
    }
}

fn setup_main_menu(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            MenuUi,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("FLAPPY BIRD"),
                TextFont { font_size: bevy::text::FontSize::Px(80.0), ..default() },
                TextColor(Color::WHITE),
            ));
            parent.spawn((
                Text::new("Press SPACE to Start"),
                TextFont { font_size: bevy::text::FontSize::Px(40.0), ..default() },
                TextColor(Color::WHITE),
            ));
        });
}

fn menu_input(keys: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if keys.just_pressed(KeyCode::Space) {
        next_state.set(GameState::Playing);
    }
}

fn setup_game_over(mut commands: Commands, save_data: Res<SaveData>) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            MenuUi,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("GAME OVER"),
                TextFont { font_size: bevy::text::FontSize::Px(80.0), ..default() },
                TextColor(Color::srgb(1.0, 0.2, 0.2)),
            ));
            parent.spawn((
                Text::new(format!("High Score: {}", save_data.high_score)),
                TextFont { font_size: bevy::text::FontSize::Px(40.0), ..default() },
                TextColor(Color::WHITE),
            ));
            parent.spawn((
                Text::new("Press SPACE to Restart"),
                TextFont { font_size: bevy::text::FontSize::Px(30.0), ..default() },
                TextColor(Color::WHITE),
            ));
        });
}

fn game_over_input(keys: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if keys.just_pressed(KeyCode::Space) {
        next_state.set(GameState::Playing);
    }
}

fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<MenuUi>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn setup_hud(mut commands: Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(20.0),
                left: Val::Percent(50.0),
                ..default()
            },
            ScoreText,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("0"),
                TextFont { font_size: bevy::text::FontSize::Px(60.0), ..default() },
                TextColor(Color::WHITE),
            ));
        });
}

fn update_hud(score: Res<Score>, mut query: Query<&mut Text, With<ScoreText>>) {
    if score.is_changed() {
        for mut text in query.iter_mut() {
            text.0 = score.0.to_string();
        }
    }
}

fn cleanup_hud(mut commands: Commands, query: Query<Entity, With<ScoreText>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
