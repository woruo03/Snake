use bevy::prelude::*;

use crate::game::{
    components::{GameOverText, HighScoreText, MainCamera, MenuUi, PerfOverlay, ScoreText},
    constants::*,
    resources::{BoardBounds, GameConfig, GameStatus},
};

pub(crate) fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, MainCamera));
}

pub(crate) fn setup_hud(mut commands: Commands) {
    commands.spawn((
        Text::new("Score: 0"),
        TextFont {
            font_size: 30.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: px(10.0),
            left: px(10.0),
            ..default()
        },
        ScoreText,
    ));

    commands.spawn((
        Text::new("High Score: 0"),
        TextFont {
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::srgb(0.95, 0.8, 0.26)),
        Node {
            position_type: PositionType::Absolute,
            top: px(44.0),
            left: px(10.0),
            ..default()
        },
        HighScoreText,
    ));
}

pub(crate) fn setup_game_over_text(mut commands: Commands) {
    commands.spawn((
        Text::new(""),
        TextFont {
            font_size: 42.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.25, 0.25)),
        Node {
            position_type: PositionType::Absolute,
            top: percent(33.0),
            left: percent(19.0),
            ..default()
        },
        Visibility::Hidden,
        GameOverText,
    ));
}

pub(crate) fn setup_perf_overlay(mut commands: Commands) {
    commands.spawn((
        Text::new(""),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        TextColor(Color::srgb(0.86, 0.92, 1.0)),
        Node {
            position_type: PositionType::Absolute,
            top: px(10.0),
            right: px(12.0),
            ..default()
        },
        Visibility::Hidden,
        PerfOverlay,
    ));
}

pub(crate) fn spawn_background_grid(
    mut commands: Commands,
    bounds: Res<BoardBounds>,
    config: Res<GameConfig>,
) {
    let width = bounds.width() as f32 * config.grid_size;
    let height = bounds.height() as f32 * config.grid_size;

    for x in bounds.min_x..=bounds.max_x {
        commands.spawn((
            Sprite::from_color(GRID_COLOR, Vec2::new(1.0, height)),
            Transform::from_translation(Vec3::new(x as f32 * config.grid_size, 0.0, Z_GRID)),
        ));
    }

    for y in bounds.min_y..=bounds.max_y {
        commands.spawn((
            Sprite::from_color(GRID_COLOR, Vec2::new(width, 1.0)),
            Transform::from_translation(Vec3::new(0.0, y as f32 * config.grid_size, Z_GRID)),
        ));
    }
}

pub(crate) fn spawn_menu_ui(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: percent(100.0),
                height: percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgba(0.02, 0.02, 0.02, 0.82)),
            MenuUi,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("SNAKE"),
                TextFont {
                    font_size: 76.0,
                    ..default()
                },
                TextColor(Color::srgb(0.18, 0.88, 0.42)),
            ));

            parent.spawn((
                Text::new(
                    "Press Enter / Space to Start\nArrow / WASD: Move\nR: Restart   F3: Perf   Esc: Quit",
                ),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::top(px(20.0)),
                    ..default()
                },
            ));
        });
}

pub(crate) fn cleanup_menu_ui(mut commands: Commands, query: Query<Entity, With<MenuUi>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

pub(crate) fn show_game_over_text(
    mut query: Query<(&mut Text, &mut Visibility), With<GameOverText>>,
    status: Res<GameStatus>,
) {
    if let Ok((mut text, mut visibility)) = query.single_mut() {
        text.0 = format!(
            "Game Over\nFinal Score: {}\nHigh Score: {}\nPress R to Restart",
            status.score, status.high_score
        );
        *visibility = Visibility::Visible;
    }
}

pub(crate) fn hide_game_over_text(mut query: Query<&mut Visibility, With<GameOverText>>) {
    if let Ok(mut visibility) = query.single_mut() {
        *visibility = Visibility::Hidden;
    }
}
