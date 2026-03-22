use bevy::prelude::*;

use crate::game::constants::{GRID_SIZE, INITIAL_SNAKE_LENGTH, WINDOW_HEIGHT, WINDOW_WIDTH};

#[derive(Resource)]
pub(crate) struct GameConfig {
    pub(crate) grid_size: f32,
    pub(crate) initial_snake_length: usize,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            grid_size: GRID_SIZE,
            initial_snake_length: INITIAL_SNAKE_LENGTH,
        }
    }
}

#[derive(Resource, Default)]
pub(crate) struct GameStatus {
    pub(crate) is_game_over: bool,
    pub(crate) score: u32,
    pub(crate) high_score: u32,
}

#[derive(Resource, Debug, Clone, Copy)]
pub(crate) struct BoardBounds {
    pub(crate) min_x: i32,
    pub(crate) max_x: i32,
    pub(crate) min_y: i32,
    pub(crate) max_y: i32,
}

impl BoardBounds {
    pub(crate) fn from_window() -> Self {
        let half_w_cells = (WINDOW_WIDTH / GRID_SIZE / 2.0).floor() as i32 - 1;
        let half_h_cells = (WINDOW_HEIGHT / GRID_SIZE / 2.0).floor() as i32 - 1;

        Self {
            min_x: -half_w_cells,
            max_x: half_w_cells,
            min_y: -half_h_cells,
            max_y: half_h_cells,
        }
    }

    pub(crate) fn width(&self) -> i32 {
        self.max_x - self.min_x + 1
    }

    pub(crate) fn height(&self) -> i32 {
        self.max_y - self.min_y + 1
    }
}

#[derive(Resource, Default)]
pub(crate) struct SnakeBody(pub(crate) Vec<Entity>);

#[derive(Resource, Default)]
pub(crate) struct PendingGrowth(pub(crate) u32);

#[derive(Resource)]
pub(crate) struct RngState {
    state: u64,
}

impl RngState {
    pub(crate) fn seeded(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next_u32(&mut self) -> u32 {
        // xorshift64*
        self.state ^= self.state >> 12;
        self.state ^= self.state << 25;
        self.state ^= self.state >> 27;
        ((self.state.wrapping_mul(0x2545_F491_4F6C_DD1D)) >> 32) as u32
    }

    pub(crate) fn range_i32(&mut self, min: i32, max: i32) -> i32 {
        if min >= max {
            return min;
        }
        let span = (max - min + 1) as u32;
        min + (self.next_u32() % span) as i32
    }

    pub(crate) fn range_f32(&mut self, min: f32, max: f32) -> f32 {
        if min >= max {
            return min;
        }
        let v = self.next_u32() as f32 / u32::MAX as f32;
        min + (max - min) * v
    }
}

#[derive(Resource, Default)]
pub(crate) struct PerfStats {
    pub(crate) show_overlay: bool,
    pub(crate) frame_count: u32,
    pub(crate) accum_seconds: f32,
    pub(crate) fps: f32,
    pub(crate) entity_count: usize,
    pub(crate) snake_count: usize,
    pub(crate) effect_count: usize,
}

#[derive(Resource, Default)]
pub(crate) struct CameraShake {
    pub(crate) timer: Option<Timer>,
    pub(crate) intensity: f32,
}
