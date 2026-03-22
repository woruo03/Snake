use bevy::prelude::*;

use crate::game::state::Direction;

#[derive(Component)]
pub(crate) struct SnakeHead {
    pub(crate) direction: Direction,
    pub(crate) next_direction: Direction,
}

#[derive(Component)]
pub(crate) struct SnakeSegment;

#[derive(Component)]
pub(crate) struct Food;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct GridPos {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

#[derive(Component)]
pub(crate) struct MenuUi;

#[derive(Component)]
pub(crate) struct ScoreText;

#[derive(Component)]
pub(crate) struct HighScoreText;

#[derive(Component)]
pub(crate) struct GameOverText;

#[derive(Component)]
pub(crate) struct PerfOverlay;

#[derive(Component)]
pub(crate) struct MainCamera;

#[derive(Component)]
pub(crate) struct GameEntity;

#[derive(Component)]
pub(crate) struct EffectEntity;

#[derive(Component)]
pub(crate) struct Lifetime {
    pub(crate) timer: Timer,
}

#[derive(Component)]
pub(crate) struct PulseEffect {
    pub(crate) t: f32,
    pub(crate) base: f32,
    pub(crate) amplitude: f32,
    pub(crate) speed: f32,
}

#[derive(Component)]
pub(crate) struct FadeEffect {
    pub(crate) alpha: f32,
    pub(crate) speed: f32,
}

#[derive(Component)]
pub(crate) struct UiFloatEffect {
    pub(crate) top_px: f32,
    pub(crate) speed: f32,
}
