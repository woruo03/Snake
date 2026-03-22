use bevy::prelude::*;

pub(crate) const WINDOW_WIDTH: f32 = 800.0;
pub(crate) const WINDOW_HEIGHT: f32 = 600.0;

pub(crate) const GRID_SIZE: f32 = 20.0;
pub(crate) const CELL_FILL: f32 = 0.9;
pub(crate) const SNAKE_FIXED_HZ: f64 = 8.0;
pub(crate) const INITIAL_SNAKE_LENGTH: usize = 3;

pub(crate) const Z_GRID: f32 = -10.0;
pub(crate) const Z_TRAIL: f32 = 0.5;
pub(crate) const Z_FOOD: f32 = 1.0;
pub(crate) const Z_SNAKE_BODY: f32 = 2.0;
pub(crate) const Z_SNAKE_HEAD: f32 = 3.0;
pub(crate) const Z_EFFECT: f32 = 4.0;

pub(crate) const BG_COLOR: Color = Color::srgb(0.08, 0.09, 0.1);
pub(crate) const GRID_COLOR: Color = Color::srgba(0.22, 0.24, 0.27, 0.28);
pub(crate) const SNAKE_HEAD_COLOR: Color = Color::srgb(0.18, 0.88, 0.42);
pub(crate) const SNAKE_SEGMENT_COLOR: Color = Color::srgb(0.12, 0.72, 0.34);
pub(crate) const FOOD_COLOR: Color = Color::srgb(0.9, 0.22, 0.26);
pub(crate) const TRAIL_COLOR: Color = Color::srgba(0.2, 0.82, 0.38, 0.35);
pub(crate) const EAT_EFFECT_COLOR: Color = Color::srgba(0.98, 0.85, 0.25, 0.85);
pub(crate) const COLLISION_COLOR: Color = Color::srgba(1.0, 0.2, 0.2, 0.55);
